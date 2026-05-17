//! Async API for EventKit (`feature = "async"`)
//!
//! Provides `Future`-based wrappers for EventKit's completion-handler and
//! synchronous APIs so they can be used seamlessly in async Rust code.
//! The implementation is **executor-agnostic** and works with any runtime
//! (tokio, async-std, smol, pollster, etc.).
//!
//! ## APIs
//!
//! | Type | Wraps |
//! |------|-------|
//! | [`RequestAccessFuture`] | `requestFullAccessToEvents`, `requestFullAccessToReminders`, `requestWriteOnlyAccessToEvents` |
//! | [`FetchRemindersFuture`] | `fetchReminders(matching:completion:)` |
//!
//! ## Synchronous save/remove
//!
//! `EKEventStore.save` and `EKEventStore.remove` are documented-synchronous
//! on Apple's platforms (no async Swift variant exists).  [`AsyncEventStore`]
//! exposes thin `async fn` wrappers that call through to the blocking
//! implementation — they compose naturally in async code but do not offload
//! to a thread pool.  Callers that need non-blocking behavior should spawn
//! them on a blocking-task thread via their executor's equivalent of
//! `tokio::task::spawn_blocking`.
//!
//! ## Tier-2 note
//!
//! `EKEventStore` change notifications (posted via `NSNotificationCenter`)
//! are a **multi-fire** stream and belong in a Tier-2 `Stream` wrapper —
//! not this Future-based module.

use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::error::EventKitError;
use crate::event::EKEvent;
use crate::event_store::{EKEventStore, EKReminderPredicate, EKSpan};
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};
use crate::reminder::EKReminder;

// ── Shared access-request callback ────────────────────────────────────────────
//
// Callback convention (all three access thunks share this):
//   result non-null (0x1) + error null  →  granted = true
//   result null            + error null →  granted = false (denied)
//   result null            + error cstr →  error
extern "C" fn access_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        let granted = !result.is_null();
        unsafe { AsyncCompletion::complete_ok(ctx, granted) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<bool>::complete_err(ctx, msg) };
    }
}

/// Future returned by all three access-request methods on [`AsyncEventStore`].
///
/// Resolves to:
/// - `Ok(true)` — access was granted by the user.
/// - `Ok(false)` — access was denied by the user (no framework error).
/// - `Err(_)` — a framework error occurred.
pub struct RequestAccessFuture {
    inner: AsyncCompletionFuture<bool>,
}

impl std::fmt::Debug for RequestAccessFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestAccessFuture").finish_non_exhaustive()
    }
}

impl Future for RequestAccessFuture {
    type Output = Result<bool, EventKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(EventKitError::OperationFailed))
    }
}

// ── fetchReminders callback ────────────────────────────────────────────────────
//
// On success, `result` is a `strdup`-allocated JSON C string produced by the
// Swift thunk.  We cast it to `*mut c_char`, deserialize, then free it via the
// existing `ek_string_free` helper (called inside `parse_json_ptr`).
extern "C" fn fetch_reminders_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        if result.is_null() {
            // null result + null error → EventKit returned nil reminders list; treat as empty.
            unsafe { AsyncCompletion::<Vec<EKReminder>>::complete_ok(ctx, vec![]) };
        } else {
            // SAFETY: the Swift thunk allocated this with `strdup`; `parse_json_ptr`
            // reads it and frees it via `ek_string_free`.
            let json_ptr = result as *mut core::ffi::c_char;
            match unsafe { parse_json_ptr::<Vec<EKReminder>>(json_ptr, "fetchReminders") } {
                Ok(reminders) => unsafe { AsyncCompletion::complete_ok(ctx, reminders) },
                Err(err) => unsafe {
                    AsyncCompletion::<Vec<EKReminder>>::complete_err(ctx, err.to_string());
                },
            }
        }
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<EKReminder>>::complete_err(ctx, msg) };
    }
}

/// Future returned by [`AsyncEventStore::fetch_reminders`].
///
/// Resolves to `Ok(Vec<EKReminder>)` (possibly empty) or `Err(_)` on failure.
pub struct FetchRemindersFuture {
    inner: AsyncCompletionFuture<Vec<EKReminder>>,
}

impl std::fmt::Debug for FetchRemindersFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FetchRemindersFuture").finish_non_exhaustive()
    }
}

impl Future for FetchRemindersFuture {
    type Output = Result<Vec<EKReminder>, EventKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(EventKitError::OperationFailed))
    }
}

// ── AsyncEventStore ────────────────────────────────────────────────────────────

/// Async-first facade over [`EKEventStore`].
///
/// Wraps every completion-handler API as a `Future` newtype and exposes the
/// synchronous `save`/`remove` operations as `async fn` for ergonomic use in
/// async contexts.
///
/// # Example
///
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # pollster::block_on(async {
/// use eventkit::async_api::AsyncEventStore;
/// use eventkit::event_store::EKEventStore;
///
/// let store = EKEventStore::new()?;
/// let async_store = AsyncEventStore::new(store);
/// let granted = async_store.request_full_access_to_events().await?;
/// println!("granted: {granted}");
/// # Ok(())
/// # })
/// # }
/// ```
pub struct AsyncEventStore {
    store: EKEventStore,
}

impl std::fmt::Debug for AsyncEventStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncEventStore").finish_non_exhaustive()
    }
}

impl AsyncEventStore {
    /// Wrap an existing [`EKEventStore`].
    pub const fn new(store: EKEventStore) -> Self {
        Self { store }
    }

    /// Borrow the inner synchronous [`EKEventStore`].
    pub const fn inner(&self) -> &EKEventStore {
        &self.store
    }

    /// Unwrap into the inner [`EKEventStore`].
    pub fn into_inner(self) -> EKEventStore {
        self.store
    }

    // ── Access requests ────────────────────────────────────────────────────────

    /// Request full calendar-events read+write access asynchronously.
    ///
    /// On macOS 14.0+ delegates to `requestFullAccessToEvents() async throws`;
    /// on earlier systems falls back to the completion-handler variant.
    pub fn request_full_access_to_events(&self) -> RequestAccessFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::event_store::ek_store_request_full_access_events_async(
                self.store.as_raw_ptr(),
                access_cb,
                ctx,
            );
        }
        RequestAccessFuture { inner: future }
    }

    /// Request full reminders read+write access asynchronously.
    ///
    /// On macOS 14.0+ delegates to `requestFullAccessToReminders() async throws`.
    pub fn request_full_access_to_reminders(&self) -> RequestAccessFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::event_store::ek_store_request_full_access_reminders_async(
                self.store.as_raw_ptr(),
                access_cb,
                ctx,
            );
        }
        RequestAccessFuture { inner: future }
    }

    /// Request write-only calendar-events access asynchronously.
    ///
    /// On macOS 14.0+ delegates to `requestWriteOnlyAccessToEvents() async throws`;
    /// falls back to full-access on earlier systems.
    pub fn request_write_only_access_to_events(&self) -> RequestAccessFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::event_store::ek_store_request_write_only_access_events_async(
                self.store.as_raw_ptr(),
                access_cb,
                ctx,
            );
        }
        RequestAccessFuture { inner: future }
    }

    // ── Fetch reminders ────────────────────────────────────────────────────────

    /// Fetch reminders matching `predicate` asynchronously.
    ///
    /// Delegates to `EKEventStore.fetchReminders(matching:completion:)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the predicate cannot be JSON-encoded, or if the
    /// Swift side reports a failure.
    pub fn fetch_reminders(
        &self,
        predicate: &EKReminderPredicate,
    ) -> Result<FetchRemindersFuture, EventKitError> {
        let predicate_json = json_cstring(predicate, "EKReminderPredicate")?;
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::event_store::ek_store_fetch_reminders_async(
                self.store.as_raw_ptr(),
                predicate_json.as_ptr(),
                fetch_reminders_cb,
                ctx,
            );
        }
        Ok(FetchRemindersFuture { inner: future })
    }

    // ── Synchronous save/remove wrapped as async fn ────────────────────────────
    //
    // EventKit's save/remove are synchronous on all Apple platforms — no async
    // Swift variant exists.  These thin wrappers let callers `.await` them in
    // async contexts.  They do NOT offload to a thread pool; use your
    // executor's spawn_blocking if you need that.

    /// Save an event (synchronous under the hood; wraps [`EKEventStore::save_event`]).
    ///
    /// # Errors
    ///
    /// Propagates any error from the synchronous implementation.
    pub fn save_event(
        &self,
        event: &EKEvent,
        span: EKSpan,
        commit: bool,
    ) -> impl Future<Output = Result<(), EventKitError>> + '_ {
        std::future::ready(self.store.save_event(event, span, commit))
    }

    /// Remove an event (synchronous under the hood; wraps [`EKEventStore::remove_event`]).
    ///
    /// # Errors
    ///
    /// Propagates any error from the synchronous implementation.
    pub fn remove_event(
        &self,
        event: &EKEvent,
        span: EKSpan,
        commit: bool,
    ) -> impl Future<Output = Result<(), EventKitError>> + '_ {
        std::future::ready(self.store.remove_event(event, span, commit))
    }

    /// Save a reminder (synchronous under the hood; wraps [`EKEventStore::save_reminder`]).
    ///
    /// # Errors
    ///
    /// Propagates any error from the synchronous implementation.
    pub fn save_reminder(
        &self,
        reminder: &EKReminder,
        commit: bool,
    ) -> impl Future<Output = Result<(), EventKitError>> + '_ {
        std::future::ready(self.store.save_reminder(reminder, commit))
    }

    /// Remove a reminder (synchronous under the hood; wraps [`EKEventStore::remove_reminder`]).
    ///
    /// # Errors
    ///
    /// Propagates any error from the synchronous implementation.
    pub fn remove_reminder(
        &self,
        reminder: &EKReminder,
        commit: bool,
    ) -> impl Future<Output = Result<(), EventKitError>> + '_ {
        std::future::ready(self.store.remove_reminder(reminder, commit))
    }
}

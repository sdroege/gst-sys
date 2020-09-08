// This file was generated by gir (https://github.com/gtk-rs/gir @ 60cbef0)
// from gir-files (https://github.com/gtk-rs/gir-files @ 989a7a8)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gstreamer_sys as gst;
extern crate libc;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Callbacks
pub type GstHarnessPrepareBufferFunc =
    Option<unsafe extern "C" fn(*mut GstHarness, gpointer) -> *mut gst::GstBuffer>;
pub type GstHarnessPrepareEventFunc =
    Option<unsafe extern "C" fn(*mut GstHarness, gpointer) -> *mut gst::GstEvent>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstHarness {
    pub element: *mut gst::GstElement,
    pub srcpad: *mut gst::GstPad,
    pub sinkpad: *mut gst::GstPad,
    pub src_harness: *mut GstHarness,
    pub sink_harness: *mut GstHarness,
    pub priv_: *mut GstHarnessPrivate,
}

impl ::std::fmt::Debug for GstHarness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstHarness @ {:?}", self as *const _))
            .field("element", &self.element)
            .field("srcpad", &self.srcpad)
            .field("sinkpad", &self.sinkpad)
            .field("src_harness", &self.src_harness)
            .field("sink_harness", &self.sink_harness)
            .finish()
    }
}

#[repr(C)]
pub struct _GstHarnessPrivate(c_void);

pub type GstHarnessPrivate = *mut _GstHarnessPrivate;

#[repr(C)]
pub struct _GstHarnessThread(c_void);

pub type GstHarnessThread = *mut _GstHarnessThread;

#[repr(C)]
pub struct _GstStreamConsistency(c_void);

pub type GstStreamConsistency = *mut _GstStreamConsistency;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstTestClockClass {
    pub parent_class: gst::GstClockClass,
}

impl ::std::fmt::Debug for GstTestClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTestClockClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GstTestClockPrivate(c_void);

pub type GstTestClockPrivate = *mut _GstTestClockPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstTestClock {
    pub parent: gst::GstClock,
    pub priv_: *mut GstTestClockPrivate,
}

impl ::std::fmt::Debug for GstTestClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTestClock @ {:?}", self as *const _))
            .field("parent", &self.parent)
            .finish()
    }
}

extern "C" {

    //=========================================================================
    // GstHarness
    //=========================================================================
    pub fn gst_harness_add_element_full(
        h: *mut GstHarness,
        element: *mut gst::GstElement,
        hsrc: *mut gst::GstStaticPadTemplate,
        element_sinkpad_name: *const c_char,
        hsink: *mut gst::GstStaticPadTemplate,
        element_srcpad_name: *const c_char,
    );
    pub fn gst_harness_add_element_sink_pad(h: *mut GstHarness, sinkpad: *mut gst::GstPad);
    pub fn gst_harness_add_element_src_pad(h: *mut GstHarness, srcpad: *mut gst::GstPad);
    pub fn gst_harness_add_parse(h: *mut GstHarness, launchline: *const c_char);
    pub fn gst_harness_add_probe(
        h: *mut GstHarness,
        element_name: *const c_char,
        pad_name: *const c_char,
        mask: gst::GstPadProbeType,
        callback: gst::GstPadProbeCallback,
        user_data: gpointer,
        destroy_data: glib::GDestroyNotify,
    );
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn gst_harness_add_propose_allocation_meta(
        h: *mut GstHarness,
        api: GType,
        params: *const gst::GstStructure,
    );
    pub fn gst_harness_add_sink(h: *mut GstHarness, sink_element_name: *const c_char);
    pub fn gst_harness_add_sink_harness(h: *mut GstHarness, sink_harness: *mut GstHarness);
    pub fn gst_harness_add_sink_parse(h: *mut GstHarness, launchline: *const c_char);
    pub fn gst_harness_add_src(
        h: *mut GstHarness,
        src_element_name: *const c_char,
        has_clock_wait: gboolean,
    );
    pub fn gst_harness_add_src_harness(
        h: *mut GstHarness,
        src_harness: *mut GstHarness,
        has_clock_wait: gboolean,
    );
    pub fn gst_harness_add_src_parse(
        h: *mut GstHarness,
        launchline: *const c_char,
        has_clock_wait: gboolean,
    );
    pub fn gst_harness_buffers_in_queue(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_buffers_received(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_crank_multiple_clock_waits(h: *mut GstHarness, waits: c_uint) -> gboolean;
    pub fn gst_harness_crank_single_clock_wait(h: *mut GstHarness) -> gboolean;
    pub fn gst_harness_create_buffer(h: *mut GstHarness, size: size_t) -> *mut gst::GstBuffer;
    pub fn gst_harness_dump_to_file(h: *mut GstHarness, filename: *const c_char);
    pub fn gst_harness_events_in_queue(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_events_received(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_find_element(
        h: *mut GstHarness,
        element_name: *const c_char,
    ) -> *mut gst::GstElement;
    pub fn gst_harness_get(
        h: *mut GstHarness,
        element_name: *const c_char,
        first_property_name: *const c_char,
        ...
    );
    pub fn gst_harness_get_allocator(
        h: *mut GstHarness,
        allocator: *mut *mut gst::GstAllocator,
        params: *mut gst::GstAllocationParams,
    );
    pub fn gst_harness_get_last_pushed_timestamp(h: *mut GstHarness) -> gst::GstClockTime;
    pub fn gst_harness_get_testclock(h: *mut GstHarness) -> *mut GstTestClock;
    pub fn gst_harness_play(h: *mut GstHarness);
    pub fn gst_harness_pull(h: *mut GstHarness) -> *mut gst::GstBuffer;
    pub fn gst_harness_pull_event(h: *mut GstHarness) -> *mut gst::GstEvent;
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn gst_harness_pull_until_eos(
        h: *mut GstHarness,
        buf: *mut *mut gst::GstBuffer,
    ) -> gboolean;
    pub fn gst_harness_pull_upstream_event(h: *mut GstHarness) -> *mut gst::GstEvent;
    pub fn gst_harness_push(h: *mut GstHarness, buffer: *mut gst::GstBuffer) -> gst::GstFlowReturn;
    pub fn gst_harness_push_and_pull(
        h: *mut GstHarness,
        buffer: *mut gst::GstBuffer,
    ) -> *mut gst::GstBuffer;
    pub fn gst_harness_push_event(h: *mut GstHarness, event: *mut gst::GstEvent) -> gboolean;
    pub fn gst_harness_push_from_src(h: *mut GstHarness) -> gst::GstFlowReturn;
    pub fn gst_harness_push_to_sink(h: *mut GstHarness) -> gst::GstFlowReturn;
    pub fn gst_harness_push_upstream_event(
        h: *mut GstHarness,
        event: *mut gst::GstEvent,
    ) -> gboolean;
    pub fn gst_harness_query_latency(h: *mut GstHarness) -> gst::GstClockTime;
    pub fn gst_harness_set(
        h: *mut GstHarness,
        element_name: *const c_char,
        first_property_name: *const c_char,
        ...
    );
    pub fn gst_harness_set_blocking_push_mode(h: *mut GstHarness);
    pub fn gst_harness_set_caps(h: *mut GstHarness, in_: *mut gst::GstCaps, out: *mut gst::GstCaps);
    pub fn gst_harness_set_caps_str(h: *mut GstHarness, in_: *const c_char, out: *const c_char);
    pub fn gst_harness_set_drop_buffers(h: *mut GstHarness, drop_buffers: gboolean);
    pub fn gst_harness_set_forwarding(h: *mut GstHarness, forwarding: gboolean);
    pub fn gst_harness_set_propose_allocator(
        h: *mut GstHarness,
        allocator: *mut gst::GstAllocator,
        params: *const gst::GstAllocationParams,
    );
    pub fn gst_harness_set_sink_caps(h: *mut GstHarness, caps: *mut gst::GstCaps);
    pub fn gst_harness_set_sink_caps_str(h: *mut GstHarness, str: *const c_char);
    pub fn gst_harness_set_src_caps(h: *mut GstHarness, caps: *mut gst::GstCaps);
    pub fn gst_harness_set_src_caps_str(h: *mut GstHarness, str: *const c_char);
    pub fn gst_harness_set_time(h: *mut GstHarness, time: gst::GstClockTime) -> gboolean;
    pub fn gst_harness_set_upstream_latency(h: *mut GstHarness, latency: gst::GstClockTime);
    pub fn gst_harness_sink_push_many(h: *mut GstHarness, pushes: c_int) -> gst::GstFlowReturn;
    pub fn gst_harness_src_crank_and_push_many(
        h: *mut GstHarness,
        cranks: c_int,
        pushes: c_int,
    ) -> gst::GstFlowReturn;
    pub fn gst_harness_src_push_event(h: *mut GstHarness) -> gboolean;
    pub fn gst_harness_stress_custom_start(
        h: *mut GstHarness,
        init: glib::GFunc,
        callback: glib::GFunc,
        data: gpointer,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_property_start_full(
        h: *mut GstHarness,
        name: *const c_char,
        value: *const gobject::GValue,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_buffer_start_full(
        h: *mut GstHarness,
        caps: *mut gst::GstCaps,
        segment: *const gst::GstSegment,
        buf: *mut gst::GstBuffer,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_buffer_with_cb_start_full(
        h: *mut GstHarness,
        caps: *mut gst::GstCaps,
        segment: *const gst::GstSegment,
        func: GstHarnessPrepareBufferFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_event_start_full(
        h: *mut GstHarness,
        event: *mut gst::GstEvent,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_event_with_cb_start_full(
        h: *mut GstHarness,
        func: GstHarnessPrepareEventFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_upstream_event_start_full(
        h: *mut GstHarness,
        event: *mut gst::GstEvent,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_push_upstream_event_with_cb_start_full(
        h: *mut GstHarness,
        func: GstHarnessPrepareEventFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_requestpad_start_full(
        h: *mut GstHarness,
        templ: *mut gst::GstPadTemplate,
        name: *const c_char,
        caps: *mut gst::GstCaps,
        release: gboolean,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    pub fn gst_harness_stress_statechange_start_full(
        h: *mut GstHarness,
        sleep: c_ulong,
    ) -> *mut GstHarnessThread;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn gst_harness_take_all_data(h: *mut GstHarness, size: *mut size_t) -> *mut u8;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn gst_harness_take_all_data_as_buffer(h: *mut GstHarness) -> *mut gst::GstBuffer;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn gst_harness_take_all_data_as_bytes(h: *mut GstHarness) -> *mut glib::GBytes;
    pub fn gst_harness_teardown(h: *mut GstHarness);
    pub fn gst_harness_try_pull(h: *mut GstHarness) -> *mut gst::GstBuffer;
    pub fn gst_harness_try_pull_event(h: *mut GstHarness) -> *mut gst::GstEvent;
    pub fn gst_harness_try_pull_upstream_event(h: *mut GstHarness) -> *mut gst::GstEvent;
    pub fn gst_harness_upstream_events_in_queue(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_upstream_events_received(h: *mut GstHarness) -> c_uint;
    pub fn gst_harness_use_systemclock(h: *mut GstHarness);
    pub fn gst_harness_use_testclock(h: *mut GstHarness);
    pub fn gst_harness_wait_for_clock_id_waits(
        h: *mut GstHarness,
        waits: c_uint,
        timeout: c_uint,
    ) -> gboolean;
    pub fn gst_harness_new(element_name: *const c_char) -> *mut GstHarness;
    pub fn gst_harness_new_empty() -> *mut GstHarness;
    pub fn gst_harness_new_full(
        element: *mut gst::GstElement,
        hsrc: *mut gst::GstStaticPadTemplate,
        element_sinkpad_name: *const c_char,
        hsink: *mut gst::GstStaticPadTemplate,
        element_srcpad_name: *const c_char,
    ) -> *mut GstHarness;
    pub fn gst_harness_new_parse(launchline: *const c_char) -> *mut GstHarness;
    pub fn gst_harness_new_with_element(
        element: *mut gst::GstElement,
        element_sinkpad_name: *const c_char,
        element_srcpad_name: *const c_char,
    ) -> *mut GstHarness;
    pub fn gst_harness_new_with_padnames(
        element_name: *const c_char,
        element_sinkpad_name: *const c_char,
        element_srcpad_name: *const c_char,
    ) -> *mut GstHarness;
    pub fn gst_harness_new_with_templates(
        element_name: *const c_char,
        hsrc: *mut gst::GstStaticPadTemplate,
        hsink: *mut gst::GstStaticPadTemplate,
    ) -> *mut GstHarness;
    pub fn gst_harness_stress_thread_stop(t: *mut GstHarnessThread) -> c_uint;

    //=========================================================================
    // GstTestClock
    //=========================================================================
    pub fn gst_test_clock_get_type() -> GType;
    pub fn gst_test_clock_new() -> *mut gst::GstClock;
    pub fn gst_test_clock_new_with_start_time(start_time: gst::GstClockTime) -> *mut gst::GstClock;
    pub fn gst_test_clock_id_list_get_latest_time(
        pending_list: *const glib::GList,
    ) -> gst::GstClockTime;
    pub fn gst_test_clock_advance_time(test_clock: *mut GstTestClock, delta: gst::GstClockTimeDiff);
    pub fn gst_test_clock_crank(test_clock: *mut GstTestClock) -> gboolean;
    pub fn gst_test_clock_get_next_entry_time(test_clock: *mut GstTestClock) -> gst::GstClockTime;
    pub fn gst_test_clock_has_id(test_clock: *mut GstTestClock, id: gst::GstClockID) -> gboolean;
    pub fn gst_test_clock_peek_id_count(test_clock: *mut GstTestClock) -> c_uint;
    pub fn gst_test_clock_peek_next_pending_id(
        test_clock: *mut GstTestClock,
        pending_id: *mut gst::GstClockID,
    ) -> gboolean;
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn gst_test_clock_process_id(
        test_clock: *mut GstTestClock,
        pending_id: gst::GstClockID,
    ) -> gboolean;
    pub fn gst_test_clock_process_id_list(
        test_clock: *mut GstTestClock,
        pending_list: *const glib::GList,
    ) -> c_uint;
    pub fn gst_test_clock_process_next_clock_id(test_clock: *mut GstTestClock) -> gst::GstClockID;
    pub fn gst_test_clock_set_time(test_clock: *mut GstTestClock, new_time: gst::GstClockTime);
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn gst_test_clock_timed_wait_for_multiple_pending_ids(
        test_clock: *mut GstTestClock,
        count: c_uint,
        timeout_ms: c_uint,
        pending_list: *mut *mut glib::GList,
    ) -> gboolean;
    pub fn gst_test_clock_wait_for_multiple_pending_ids(
        test_clock: *mut GstTestClock,
        count: c_uint,
        pending_list: *mut *mut glib::GList,
    );
    pub fn gst_test_clock_wait_for_next_pending_id(
        test_clock: *mut GstTestClock,
        pending_id: *mut gst::GstClockID,
    );
    pub fn gst_test_clock_wait_for_pending_id_count(test_clock: *mut GstTestClock, count: c_uint);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_consistency_checker_add_pad(
        consist: *mut GstStreamConsistency,
        pad: *mut gst::GstPad,
    ) -> gboolean;
    pub fn gst_consistency_checker_free(consist: *mut GstStreamConsistency);
    pub fn gst_consistency_checker_new(pad: *mut gst::GstPad) -> *mut GstStreamConsistency;
    pub fn gst_consistency_checker_reset(consist: *mut GstStreamConsistency);

}

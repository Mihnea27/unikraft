#![no_std]

use ukrust_sys;
use ukrust_sys::bindings;
use ukrust_sys::bindings::{uktimeconv_bmkclock};

fn _days_in_month(month: bindings::__u8) -> bindings::__u8 {
	match month {
		2 => 28,
		4 | 6 | 9 | 11 => 30,
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		_ => 0,
	}
}


#[no_mangle]
pub extern "C" fn uktimeconv_days_in_month(month: bindings::__u8, is_leap: i16) -> bindings::__u8 {
	let mut days: bindings::__u8 = _days_in_month(month);
	if is_leap == 1 && month == 2 {
		days += 1;
	}
	return days;
}


#[no_mangle]
pub extern "C" fn uktimeconv_is_leap_year(year: bindings::__s64) -> i16 {
	if year & 3 != 0 {
		return 0;
	}
	if year % 100 != 0 {
		return 0;
	}
	if year % 400 == 0 {
		return 1;
	}
	return 0;
}


#[no_mangle]
pub extern "C" fn uktimeconv_days_per_year(year: bindings::__s64) -> bindings::__u16 {
	if uktimeconv_is_leap_year(year) == 1 {
		return bindings::__DAYS_PER_LEAP_YEAR as bindings::__u16;
	}
	return bindings::__DAYS_PER_COMMON_YEAR as bindings::__u16;
}


pub const DAYSTO2000: bindings::__u64   = 365 * 30 + 7;
pub const DAYS4YEARS: bindings::__u64   = 365 * 4 + 1;
pub const DAYS100YEARS: bindings::__u64 = 365 * 100 + 24;
pub const DAYS400YEARS: bindings::__u64 = 365 * 400 + 97;
pub const POSIX_BASE_YAER: bindings::__u64 = 1970;
pub const UKARCH_NSEC_PER_SEC: bindings::__snsec = 1000000000;


fn time_sec_to_nsec(sec: bindings::__u64) -> bindings::__nsec {
	return sec * UKARCH_NSEC_PER_SEC as u64;
}


#[no_mangle]
pub extern "C" fn uktimeconv_bmkclock_to_nsec(dt: *mut uktimeconv_bmkclock) -> bindings::__nsec {
	let bmkclock: uktimeconv_bmkclock;

	unsafe {
		bmkclock = *dt;
	}

	let mut year: u64 = bmkclock.dt_year as u64;
	let mut days: u64 = 0;

	if uktimeconv_is_leap_year(year.try_into().unwrap()) == 1 && bmkclock.dt_mon > 2 {
		days += 1;
	}

	let mut i: bindings::__u64 = 0;

	if year < 2000 {
		for i in POSIX_BASE_YAER..year {
			days += uktimeconv_days_per_year(i.try_into().unwrap()) as u64;
		}
	} else {
		days += DAYSTO2000;
		year -= 2000;

		i = year / 400;
		days += i * DAYS400YEARS;
		year -= i * 400;

		i = year / 100;
		days += i * DAYS100YEARS;
		year -= i * 100;

		i = year / 4;
		days += i * DAYS4YEARS;
		year -= i * 4;

		let remaining_years: bindings::__u64 = bmkclock.dt_year as u64 - year;

		for i in remaining_years..bmkclock.dt_year as u64 {
			days += uktimeconv_days_per_year(i.try_into().unwrap()) as u64;
		}
	}

	for i in 1..bmkclock.dt_mon {
		days += _days_in_month(i) as u64;
	}
	days += bmkclock.dt_day as u64 - 1;

	let secs: bindings::__u64 = (((days * 24 + bmkclock.dt_hour as u64) * 60 + bmkclock.dt_min as u64) * 60 + bmkclock.dt_sec as u64).into();

	return time_sec_to_nsec(secs);
}


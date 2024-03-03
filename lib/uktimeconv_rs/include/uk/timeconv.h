#ifndef __UKTIMECONV_H__
#define __UKTIMECONV_H__

#include <uk/arch/time.h>

#ifdef	__cplusplus
extern "C" {
#endif

/* Some handy constants. */
#define __SECS_PER_MINUTE        60
#define __SECS_PER_HOUR          3600
#define __SECS_PER_DAY           86400
#define __DAYS_PER_COMMON_YEAR   365
#define __DAYS_PER_LEAP_YEAR     366

struct uktimeconv_bmkclock {
	__s64 dt_year;
	__u8  dt_mon;
	__u8  dt_day;
	__u8  dt_hour;
	__u8  dt_min;
	__u8  dt_sec;
};

extern int uktimeconv_is_leap_year(__s64 year);
extern __u8 uktimeconv_days_in_month(__u8 month, int is_leap_year);

extern __u16 uktimeconv_days_per_year(__s64 year);

/*
 * "POSIX time" from "YY/MM/DD/hh/mm/ss"
 */
extern __nsec uktimeconv_bmkclock_to_nsec(struct uktimeconv_bmkclock *dt);

/*
 * BCD to binary.
 */
static inline unsigned int uktimeconv_bcdtobin(unsigned int bcd)
{
	return ((bcd >> 4) & 0x0f) * 10 + (bcd & 0x0f);
}

#ifdef	__cplusplus
}
#endif

#endif /* __UKTIMECONV_H__ */

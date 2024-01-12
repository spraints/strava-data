# Strava Data

This is where I download and play with my Strava activity data.

## Synopsis

1. [Download your data.](https://support.strava.com/hc/en-us/articles/216918437-Exporting-your-Data-and-Bulk-Export#h_01GG58HC4F1BGQ9PQZZVANN6WF) (Note: you don't need to delete your account, even though the page looks like it's geared towards that.)
2. Extract the archive. You'll end up with a directory called something like `export_7353329`.
3. `strava-data trend --dir ARCHIVE --activity ACTIVITY_TYPE --metric METRIC`.

Example:

```
$ strava-data trend --dir export_7353329 --activity yoga --metric duration
Date                 moving    elapsed
2024-01-02 19:45      26:21      26:21
2024-01-03 20:44      21:34      21:34
2024-01-05 20:29      21:02      21:02
2024-01-06 20:34      15:34      15:34
```

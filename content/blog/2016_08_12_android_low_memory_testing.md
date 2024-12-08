+++
title = "Android low memory testing"
date = 2016-08-12

[taxonomies]
tag = ["Android", "code"]

[extra]
id = "blog-single"
+++

Resource constraints is what separate mobile development from other types of development.
This post focuses on "low memory" constraints and to test for this ephemeral state.
<!-- more -->

### Out of sight, out of mind
It is often easy to ignore a problem that is unlikely to occur. Of course as the consumer
market steadily moves towards more powerful phones, low memory issue should be a thing of
the past. However, if each app developer thinks along these lines then we make no
progress.

### Living on the edge: destroy everything
Enable the setting: `Settings -> Developer options -> Don't keep activities`. By checking
this option Android should "simulate" low memory behavior, as the OS will destroy an
activity as the user navigates away from it. Since cleaning up activities is what the OS
does to reclaim memory, this is a close approximation.

### Code defensively
If left un-handled, this can result in invalid state or loss of state in your application.
Luckily the Android framework provides helpers to presrve and restore state between
activity transitions. Use the method `onSaveInstanceState()` to save any variables when
leaving an activity and `onRestoreInstanceState()` to restore the variables back when
re-creating an activity.

Happy developing :)

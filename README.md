# mdt8
## A Mindfulness Meditation Tool for Tech People

`mdt8` is a tool to help those who spend lots of time in the terminal remember to stay in touch with their bodies via mindfulness meditation.

## Installation

If you use rust,


## Usage

Running `mdt8 status` will tell you what your progress is today, and what your goals are. For instance:

```
$ mdt8
You plan to spend 60 minutes per day on mindfulness.
So far, you've spent 38 minutes.
```

You can tell mdt8 that you're starting a meditation with `mdt8 start`. When you're done, simply type `mdt8 stop`.

```
$ mdt8 start
Starting timer.
Remember to breathe deeply, and relax.

<time passes>
$ mdt8 stop
Stopping timer.
```

You can also `mdt8 cancel` if you left the timer running by accident, and `mdt8 mod MINUTES` to add
a session manually.



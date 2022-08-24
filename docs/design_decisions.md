# Design Decisions
This doc cover the design decisions base9 made and why.
Discussion welcomed through issues.

## number of colors in palette
A background and a foreground is a must.

I surveyed a bunch of popular color schemes. Most of them has 7-8 hues.

Examples:
- gruvbox: 7
- dracula: 7
- solarized: 8
- monokai: 6 (maybe, since it's close sourced)
- one dark: does not fit in hue model

It's pretty arbitrary. I chose 9, since is the average.


## Restrict semantic names

Restricting semantic names to regex `[a-z][0-9a-z_]*`.

This should support all names, but
enable more flexible language binding.


## Centralized Template Repo

Benefits:
1. easier to write template config.
  a. Only need to specify path, no need to specify URL.
2. supports mustache partials easily
3. easier for manager to update template files.
4. easier to make sure base9 style, api are in sync.
5. support easier install method for individual app.

Drawbacks:
1. have to maintain mustache in two places.
2. Delayed template update.
3. More work to auto update templates.

Decision: Yes

## Keep simple templates in the centralized repo

Benefits:
1. Easier to sync/update

Drawbacks:
1. More than one purpose for one repo, harder to track issues and changes.
2. README and other unrelated things will be in the repo.

Decision: No


## Add variable in list form to enable themes.

### With this, vim theme would look like:

a mustache with:
```
c0 = #121212
field.p10 = #244924
...
```
And many files using those variables

Manager would just update one file

The vim plugin would

### Without this, vim theme would look like:

Many mustache files like:
```
status_bar = #{{primary.p75}}
```

And a lua script to generate all template files

Decision: Yes

## Manager vs standalone in theme plugins

Standalone flow:
1. Download plugin.
2. Set palette code in setting.

Manager flow:
1. Download plugin.
2. Make sure setting is empty.
2. Add manager config
3. run manager.



# Base9 theme for [Kitty](https://sw.kovidgoyal.net/kitty/).


<details >
<summary>View screenshots</summary>
282828-ebdbb2-83a598-8ec07c-fe8019-fabd2f-b8bb26-d3869b-fb4934
<img src="https://user-images.githubusercontent.com/2196866/180879121-6bdb8bda-07f9-4041-a36c-e30e23fd0a8c.png"/>
</details>

## Installation


Make sure `~/.config/kitty/kitty.conf` has the following:
```
include base9.conf
```


Set your Base9 palette code to shell:
```bash
BASE9_PALETTE="282828-ebdbb2-83a598-8ec07c-fe8019-fabd2f-b8bb26-d3869b-fb4934"
```

Generate theme and copy to Kitty's config:
```bash
curl -L https://github.com/base9-theme/base9-builder/releases/download/0.1/base9-builder --output base9-builder
chmod +x base9-builder

curl -L \
https://raw.githubusercontent.com/base9-theme/base9-templates/main/templates/kitty/default.conf.mustache \
--output base9.mustache

DESTINATION=~/.config/kitty/base9.yml
./base9-builder render $BASE9_PALETTE base9.mustache $DESTINATION
```

Clean up:
```bash
rm base9-builder
rm base9.mustache
```



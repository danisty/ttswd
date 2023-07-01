# Tabletob Simulator Workshop Downloader

I'm almost certain you're here for non legal reasons.

## Features
- **Desktop app** to download mods from the Workshop and manage locally installed ones.
- **CLI** to download mods only.

## Screenshots
![](https://i.imgur.com/xVKQnhI.png)
![](https://i.imgur.com/z3xbejy.png)
![](https://i.imgur.com/3cdXssf.png)

## CLI
Usage
```shell
./cli [workshop_mod_id]
```

ge. https://steamcommunity.com/sharedfiles/filedetails/?id=260389428
```shell
./cli 260389428
```

## TODO
- Add an actual icon.
- Show more information about mods.
- Set steam cookie for authorized only content (Must be logged in steam). 
  - You can use the **CLI** for athorized only mods, they only don't appear in the GUI.
- Consider more searching options / filters.
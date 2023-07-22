# Tabletop Simulator Workshop Downloader

I'm almost certain you're here for non legal reasons.

## Features
- **Desktop app** to download games from the Workshop and manage locally installed ones.
- **CLI** to download games (only).

## Screenshots
![](https://i.imgur.com/xVKQnhI.png)
![](https://i.imgur.com/z3xbejy.png)
![](https://i.imgur.com/3cdXssf.png)

## CLI
Usage
```shell
./ttswd-cli.exe [workshop_game_id]
```

ge. https://steamcommunity.com/sharedfiles/filedetails/?id=260389428
```shell
./ttswd-cli.exe 260389428
```

## TODO
- Add an actual icon.
- Show more information about games.
- Set steam cookie for authorized only content (Must be logged in steam). 
  - You can use the **CLI** for athorized only games, they only don't appear in the GUI.
- Consider more searching options / filters.

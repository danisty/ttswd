import "./App.css";

import { For, createSignal, createEffect, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { purple } from "@suid/material/colors";
import { SelectChangeEvent } from "@suid/material/Select";
import {
  createTheme,
  ThemeProvider,
  Box,
  Button,
  Icon,
  TextField,
  CardMedia,
  Typography,
  Grid,
  Card,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  CircularProgress
} from "@suid/material";

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: purple[500],
    },
    secondary: {
      main: "#11cb5f",
    },
  },
});

function App() {
  const [data, setData] = createSignal({} as any);
  const [page, setPage] = createSignal(1);
  const [searching, setSearching] = createSignal(false);
  const [reload, setReload] = createSignal(true);

  const [query, setQuery] = createSignal("");
  const [sortBy, setSortBy] = createSignal("trend");
  const [library, setLibrary] = createSignal("online");

  createEffect(() => {
    if (reload()) {
      invoke("search_workshop", {
        query: encodeURIComponent(query()),
        sort: sortBy(),
        library: library(),
        page: page()
      }).then((res) => {
        setReload(false);
        setSearching(false);
        setData(JSON.parse(res as string));
      });
    }
  });

  // EVENTS
  function onModAction(mod: any): Promise<boolean>  {
    return new Promise(r => {
      if (mod.downloaded) {
        invoke("remove_mod", { id: mod.id }).then(() => {
          mod.downloaded = false;
          r(false);
        });
      } else {
        invoke("download_mod", { id: mod.id, author: mod.author, img: mod.img }).then(() => {
          mod.downloaded = true;
          r(true);
        });
      }
    });
  }

  function onChangePage(page: number) {
    if (page < 1 || page > data().pages || reload())
      return;

    setPage(page);
    setReload(true);
  }

  // COMPOSABLES (?)
  function fromArray(arr: any[], pg: number) {
    return (
      <For each={arr}>{
        (p: number) => <Button disabled={p == pg} onClick={() => onChangePage(p)}>{p}</Button> 
      }</For>
    )
  }

  function inBetweenPages() {
    let pg = page();
    let pgs = data().pages;

    if (pgs < 6) {
      return fromArray(Array.from(Array(pgs - 2), (_, i) => i + 2), pg);
    } else if (pg > 3 && pg < pgs - 2) {
      return (
        <>
          <a class="px-4">...</a>
          {fromArray(Array.from(Array(3), (_, i) => pg + i - 1), pg)}
          <a class="px-4">...</a>
        </>
      )
    } else if (page() <= 3) {
      return (
        <>
          {fromArray(Array.from(Array(4), (_, i) => i + 2), pg)}
          <a class="px-4">...</a>
        </>
      )
    } else {
      return (
        <>
          <a class="px-4">...</a>
          {fromArray(Array.from(Array(4), (_, i) => pgs - (4 - i)), pg)}
        </>
      )
    }
  }

  function createPagesSelector() {
    return (
      <Show when={library() == "online" && data().pages > 0 && !searching()}>
        <div class="flex justify-end">
          <Button onClick={() => onChangePage(page() - 1)}>
            <Icon>navigate_before</Icon>
          </Button>
          <Button disabled={page() == 1} onClick={() => onChangePage(1)}>{1}</Button>
          
          {inBetweenPages()}

          <Button disabled={page() == data().pages} onClick={() => onChangePage(data().pages)}>{data().pages}</Button>
          <Button onClick={() => onChangePage(page() + 1)}>
            <Icon>navigate_next</Icon>
          </Button>
        </div>
      </Show>
    )
  }

  function createSelector(title: string, value: string, disabled: any, items: any, onChange: any) {
    return (
      <FormControl sx={{ m: 1, minWidth: 120 }} size="small" class="m-[0!important]">
        <InputLabel>{title}</InputLabel>
        <Select
          disabled={disabled()}
          value={value}
          label={title}
          onChange={onChange}
        >
          <For each={Object.entries(items)}>
            {(item: any) => <MenuItem value={item[1]}>{item[0]}</MenuItem>}
          </For>
        </Select>
      </FormControl>
    )
  }

  function createCard(mod: any) {
    const [downloaded, setDownloaded] = createSignal(mod.downloaded);
    const [downloading, setDownloading] = createSignal(false);
  
    return (
      <Card>
        <CardMedia
          class="h-[240px]"
          component="img"
          image={mod.img}
          alt="If you're seeing this, go watch The Ancient Magus Bride immediately ᓚᘏᗢ"
        />
        <Box class="p-2">
          <Typography gutterBottom fontSize={17} textOverflow="ellipsis">
            <div title={mod.title} class="truncate">{mod.title}</div>
          </Typography>
          <Typography variant="body2" color="text.secondary" textOverflow="ellipsis" class="pb-2">
            {mod.author}
          </Typography>
          <Button
            class="w-full h-9"
            variant="outlined"
            disabled={downloading()}
            color={downloaded() ? "secondary" : "primary"}
            onClick={() => {
              if (!mod.downloaded) setDownloading(true);
              onModAction(mod).then((downloaded: boolean) => {
                setDownloading(false);
                setDownloaded(downloaded);
              });
            }}
          >
            { downloading() ?
                <CircularProgress color="primary" size={20} />
              :
                downloaded() ? "remove" : "download" 
            }
          </Button>
        </Box>
      </Card>
    );
  }

  function buildHeader() {
    return (
      <>
        <Typography class="transition-all pt-5 text-center" fontSize={18} id="header">
          Tabletop Simulator Workshop<br/>
          <a class="text-blue-500 leading-2" href="https://github.com/danisty" target="_blank">@danisty</a>
          <a class="text-neutral-500 leading-2"> 2023</a>
        </Typography>
        <Box class="z-10 sticky top-0 w-full">
          <Box 
            backgroundColor="background.default"
            class="flex flex-col items-center max-w-3xl m-auto pt-5 pb-1 px-10"
          >
            <div class="flex flex-row gap-2 pb-2 w-full">
              <TextField
                id="search-bar"
                class="w-full"
                label="Search for a mod"
                variant="outlined"
                size="small"
                value={query()}
                onChange={(_, q) => setQuery(q)}
                onKeyDown={(e) => {
                  if (e.key == "Enter" && !reload()) {
                    setPage(1);
                    setSearching(true);
                    setReload(true);
                  }
                }}
              />
              {createSelector("Sort by", sortBy(), () => library() == "local", {
                "Popular": "trend",
                "Recent": "mostrecent",
                "Updated": "lastupdated",
                "Most subscriptions": "totaluniquesubscribers",
              }, (event: SelectChangeEvent) => {
                setSortBy(event.target.value);
                setPage(1);
                setReload(true);
              })}
              {createSelector("Library", library(), () => false, {
                "Online": "online",
                "Local": "local",
              }, (event: SelectChangeEvent) => {
                setLibrary(event.target.value);
                setPage(1);
                setQuery("");
                setReload(true);
              })}
            </div>
            {createPagesSelector()}
          </Box>
          <div class="w-full h-2 bg-gradient-to-b from-[#121113]" />
        </Box>
      </>
    );
  }

  function buildContent() {
    if (reload()) {
      return (
        <div class="flex flex-grow items-center">
          <CircularProgress color="primary" />
        </div>
      );
    } else {
      return (
        <>
          <Grid container spacing={2} columns={{ xs: 2, sm: 3, md: 3 }} paddingBottom={2}>
            {
              library() == "local" ?
                <For each={data().items.filter(
                  (x: any) => x.title.toLowerCase().includes(query().toLowerCase())
                )}>{
                  (item: any) => <Grid item xs={1}>{createCard(item)}</Grid>
                }</For>
              :
                <For each={data().items}>{
                  (item: any) => <Grid item xs={1}>{createCard(item)}</Grid>
                }</For>
            }
          </Grid>
          {createPagesSelector()}
        </>
      );
    }
  }

  return (
    <>
      <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/icon?family=Material+Icons"
      />
      <ThemeProvider theme={theme}>
        {buildHeader()}
        <div class="flex flex-col items-center max-w-3xl m-auto pb-5 px-10">
          {buildContent()}
        </div>
      </ThemeProvider>
    </>
  );
}

export default App;

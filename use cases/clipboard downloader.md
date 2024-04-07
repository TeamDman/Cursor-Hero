action: process clipboard and propose download plan
action: follow through on download plan

consider the following: https://app.suno.ai/song/6a4fb994-1732-4dcc-8262-fdbf0bdea1b0/

```html
<link rel="icon" href="/favicon.ico" sizes="any" />
<title> Bit Rockin | Suno</title ><meta name="description" content="melodic 16bit rhythm, dubstep, trap hop, hard rock, dubstep, song. Listen and make your own with Suno." />
<meta property="og:title" content="Bit Rockin | Suno" />
<meta property="og:description" content="melodic 16bit rhythm, dubstep, trap hop, hard rock, dubstep, song. Listen and make your own with Suno." />
<meta property="og:image" content="https://cdn1.suno.ai/image_256d2fa6-7759-42b8-b6ea-66986ef0177c.png" />
<meta property="og:image:width" content="256" />
<meta property="og:image:height" content="256" />
<meta property="og:image:type" content="image/png" />
<meta property="og:video:url" content="https://cdn1.suno.ai/6a4fb994-1732-4dcc-8262-fdbf0bdea1b0.mp4" />
<meta property="og:video:type" content="video/mp4" />
<meta property="og:audio" content="https://cdn1.suno.ai/6a4fb994-1732-4dcc-8262-fdbf0bdea1b0.mp3" />
<meta property="og:type" content="music.song" />
<meta name="twitter:card" content="player" />
<meta name="twitter:site" content="@suno_ai_" />
<meta name="twitter:title" content="Bit Rockin | Suno" />
<meta name="twitter:description" content="melodic 16bit rhythm, dubstep, trap hop, hard rock, dubstep, song. Listen and make your own with Suno." />
<meta name="twitter:image" content="https://cdn1.suno.ai/image_large_256d2fa6-7759-42b8-b6ea-66986ef0177c.png" />
<meta name="twitter:image" content="https://cdn1.suno.ai/image_256d2fa6-7759-42b8-b6ea-66986ef0177c.png" />
<meta name="twitter:player" content="https://app.suno.ai/embed/6a4fb994-1732-4dcc-8262-fdbf0bdea1b0/" />
<meta name="twitter:player:stream" content="https://cdn1.suno.ai/6a4fb994-1732-4dcc-8262-fdbf0bdea1b0.mp3" />
<meta name="twitter:player:width" content="760" />
<meta name="twitter:player:height" content="240" />
```

Given that URL in my clipboard, it can be fetched and HTML will be responded with.

We can extract from HTML information that will tell us how to proceed with downloading.

For this example, Suno is a AI music service which has a very simple CDN structure.

My desired download action would be to download the music file in the highest quality available, attach metadata, and store it in a folder named Suno, saving the file under a YYYY folder, then MM folder, then DD folder. This is because Windows struggles with many files in a flat structure.

---

Consider the link: https://cdn.discordapp.com/attachments/629343459869720646/1226245436441104434/aYQNpPN_460svvp9.webm?ex=6624110e&is=66119c0e&hm=aa5dd52f1f21254562d7ba360a9900dbc84eb7d57e07db66d1b0ce749e6163d5&

This is a URL copied from Discord.

This would be saved using the name from the URL and would be saved under `Discord/$YYYY/$MM/$DD`
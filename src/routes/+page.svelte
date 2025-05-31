<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let name = $state("");
  let greetMsg = $state("");
  let stream = null;

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function startCamera() {
  const video = document.getElementById('video');
  try {
    stream = await navigator.mediaDevices.getUserMedia({
       video: {
         facingMode: 'user'
       },
    });
    video.srcObject = stream;
  } catch (e) {
    console.error('Gagal mengakses kamera:', e);
  }
}

function stopCamera() {
  if (stream) {
    stream.getTracks().forEach(track => track.stop());
    const video = document.getElementById('video');
    video.srcObject = null;
    stream = null;
  }
}

let frame = $state('');

onMount( async () => {
    frame = await invoke('get_camera_frame');
  });
</script>

<main class="container">
  <h1>Welcome Bayu</h1>

  {#if frame}
    <img src={frame} alt="Live Camera Feed" style="max-width: 100%; border: 2px solid #ccc;" />
  {:else}
    <p>Loading preview...</p>
  {/if}

  <div class="row">
    <video id="video" autoplay playsinline class="camera-feed"></video>
  </div>

<div class="row">
  <button onclick={startCamera}>Start Camera</button>
  <button onclick={stopCamera}>Stop Camera</button>
</div>

  <p>Happy wedding bibir!.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-top: 10px;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

.camera-feed {
    height: 80%;

    /* NON-mirrored, arah sesuai kenyataan */
/*    transform: none;*/
  transform: scaleX(-1);
    object-fit: cover;
  }

</style>

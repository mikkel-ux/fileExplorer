<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const appWindow = getCurrentWindow();

  const minimize = () => {
    console.log("Minimizing window");

    appWindow.minimize();
  };

  const toggleMaximize = () => {
    console.log("Toggling window maximize state");
    appWindow.toggleMaximize();
  };

  const close = () => {
    console.log("Closing window");
    appWindow.close();
  };
</script>

<div
  data-tauri-drag-region
  class="text-white flex justify-between items-center p-2"
>
  <p>test</p>
  <div>
    <button on:click={minimize}>_</button>
    <button on:click={toggleMaximize}>[]</button>
    <button on:click={close}>X</button>
  </div>
</div>

<style>
</style>

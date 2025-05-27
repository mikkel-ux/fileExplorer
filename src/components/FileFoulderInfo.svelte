<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { removeSelectedFile, selectedFile } from "../stores/tabsStore";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { isImage } from "../functions/checkFileExtension";
  import { Folder, Inspect } from "@lucide/svelte";

  let autoplay = $state<boolean>(true);

  const close = () => {
    removeSelectedFile();
  };

  const getImageUrl = (path: string) => {
    return convertFileSrc(path);
  };

  const stillImage = $selectedFile?.path.replace(/\.gif$/, ".png");
</script>

<section
  in:fly={{ x: 100, duration: 200 }}
  out:fade={{ duration: 200 }}
  class="w-2xl h-full flex flex-col gap-4 p-2 bg-secondary-bg transition-colors text-white overflow-y-auto"
>
  <div class="flex justify-between">
    <p class="text-2xl font-bold">Details</p>
    <button class="font-bold hover:opacity-50" onclick={() => close()}>
      x
    </button>
  </div>
  <div
    class="border-b-4 border-t-4 border-gray-500 flex flex-col justify-center items-center p-2 gap-2"
  >
    {#if $selectedFile && $selectedFile.type === "folder"}
      <Folder size="50%" />
    {:else if $selectedFile && isImage($selectedFile)}
      {#if autoplay && $selectedFile.extension === "gif"}
        <p class="text-gray-400">No preview available</p>
      {:else}
        <img
          src={getImageUrl($selectedFile.path)}
          alt="Selected file"
          class="max-h-40 sm:max-h-60 md:max-h-80 lg:max-h-[32rem] w-auto"
        />
      {/if}
    {:else if $selectedFile && $selectedFile.extension === "pdf"}
      <iframe
        src={getImageUrl($selectedFile.path)}
        class="w-full rounded-lg h-60"
        scrolling="auto"
        title="PDF Preview"
      ></iframe>
    {:else}
      <p class="text-gray-400">No preview available</p>
    {/if}
    <p class="text-sm text-center break-all">
      {$selectedFile ? $selectedFile.name : "No file selected"}
    </p>
  </div>

  <div class="grid grid-cols-2 gap-4 text-sm">
    <div>
      <p class="text-gray-400">Type</p>
      <p class="font-semibold">
        {$selectedFile?.type === "folder"
          ? "Folder"
          : ($selectedFile?.extension ?? "–")}
      </p>
    </div>

    <div>
      <p class="text-gray-400">Size</p>
      <p class="font-semibold">{$selectedFile?.size ?? "–"}</p>
    </div>

    <div class="col-span-2">
      <p class="text-gray-400">Location</p>
      <p class="break-words font-semibold">{$selectedFile?.path ?? "–"}</p>
    </div>

    <div>
      <p class="text-gray-400">Created</p>
      <p class="font-semibold">{$selectedFile?.created ?? "–"}</p>
    </div>

    <div>
      <p class="text-gray-400">Modified</p>
      <p class="font-semibold">{$selectedFile?.modified ?? "–"}</p>
    </div>

    <div class="col-span-2">
      <p class="text-gray-400">Last Accessed</p>
      <p class="font-semibold">{$selectedFile?.accessed ?? "–"}</p>
    </div>
  </div>

  <div class="flex justify-end mt-4">
    <button class="w-full bg-folder text-black rounded-lg"> open </button>
  </div>
</section>

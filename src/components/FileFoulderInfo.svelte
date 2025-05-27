<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { removeSelectedFile, selectedFile } from "../stores/tabsStore";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";

  import { onMount } from "svelte";

  const close = () => {
    removeSelectedFile();
  };

  const getImageUrl = async (path: string) => {
    const appData = await appDataDir();
    const fullPath = await join(appData, path);
    console.log("Full path:", fullPath);

    return convertFileSrc(fullPath);
  };
  const imgUrl = convertFileSrc(
    "C:\\Users\\rumbo\\OneDrive\\Billeder\\profile_picture.png"
  );

  onMount(async () => {
    let test = await getImageUrl(
      "C:\\Users\\rumbo\\OneDrive\\Billeder\\profile_picture.png"
    );
    console.log("Test image URL:", test);
  });
</script>

<section
  in:fly={{ x: 100, duration: 200 }}
  out:fade={{ duration: 200 }}
  class="w-2xl h-full flex flex-col gap-4 p-4 bg-red-600 transition-colors text-white"
>
  <button
    class="btn btn-primary bg-blue-400 hover:bg-blue-900"
    onclick={() => close()}
  >
    x
  </button>
  <h1 class="text-2xl font-bold">
    {$selectedFile ? $selectedFile.name : "No file selected"}
  </h1>
  <p class="text-2xl font-bold">
    {$selectedFile ? $selectedFile.extension : "No file selected"}
  </p>

  {#if $selectedFile?.extension === "png" || $selectedFile?.extension === "jpg" || $selectedFile?.extension === "jpeg"}
    <!-- <img
      src={await getImageUrl($selectedFile.path)}
      alt="Selected file"
      class="max-w-full max-h-96 object-cover"
    /> -->
    <!-- <p>image</p> -->
    <img
      src="http://asset.localhost/C%3A%5CUsers%5Crumbo%5CAppData%5CRoaming%5Ccom.fileexplorer.app%5CC%3A%5CUsers%5Crumbo%5COneDrive%5CBilleder%5Cprofile_picture.png"
      alt="Selected file"
      class="max-w-full max-h-96 object-cover"
    />
  {:else}
    <p class="text-gray-300">Preview not available for this file type.</p>
  {/if}
</section>

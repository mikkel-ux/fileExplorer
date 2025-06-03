<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { updateHistory, getCurrentPath } from "../stores/tabsStore";
  import { secsectFile, selectedFile } from "../stores/tabsStore";
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { Folder, Inspect, FileText } from "@lucide/svelte";
  import { isImage } from "../functions/checkFileExtension";
  import type { FileDataType } from "../../type";

  let files = $state<FileDataType[]>([]);
  let clickTimer = $state<NodeJS.Timeout | null>(null);
  let enterTimer = $state<NodeJS.Timeout | null>(null);

  onMount(async () => {
    try {
      let result: any[] = await invoke("get_files_dirs_in_dir", {
        path: "C:\\Users\\rumbo\\OneDrive\\Billeder",
      });
      files = result;
      console.log("files", files);
    } catch (error) {
      console.log("error", error);
    }
  });

  const handleClick = (file: FileDataType) => {
    if (clickTimer) {
      clearTimeout(clickTimer);
      clickTimer = null;
      openFile(file);
    } else {
      clickTimer = setTimeout(() => {
        secsectFile(file);
        clickTimer = null;
      }, 200);
    }
  };

  const handleKeyDown = (e: KeyboardEvent, file: FileDataType) => {
    if (e.key === "Enter") {
      e.preventDefault();
      if (enterTimer) {
        clearTimeout(enterTimer);
        enterTimer = null;
        openFile(file);
      } else {
        enterTimer = setTimeout(() => {
          secsectFile(file);
          enterTimer = null;
        }, 200);
      }
    }
  };

  const openFile = async (file: FileDataType) => {
    if (file.type === "folder") {
      updateHistory(file.path);
      return;
    }
    try {
      await invoke("open_in_default_app", { path: file.path });
    } catch (error) {
      console.error("Error opening file:", error);
    }
  };

  const getImageUrl = (path: string) => {
    return convertFileSrc(path);
  };
</script>

<section class="w-full h-full overflow-y-auto overflow-x-hidden">
  <div
    class="grid grid-cols-5 grid-rows-[repeat(1fr)] gap-2 justify-items-center
  text-center pl-10 pr-10 pb-5 pt-2 select-none"
  >
    {#each files as file}
      <div
        onclick={() => handleClick(file)}
        role="button"
        tabindex="0"
        class={`ffContainer w-[15cqw] h-[20cqh] rounded-lg flex flex-col items-center justify-center hover:bg-secondary-bg
      transition-bg ease-in-out
        ${$selectedFile && $selectedFile.name === file.name ? "bg-highlight" : ""}
        ${file.isHidden ? "opacity-50" : ""}
      `}
        onkeydown={(e) => {
          handleKeyDown(e, file);
        }}
      >
        {#if file.type === "folder"}
          <Folder size="50%" />
        {:else if file.extension.toLowerCase() === "txt"}
          <FileText size="50%" />
        {:else if isImage(file)}
          {#if file.extension === "gif" && file.base64}
            <img
              src={`data:image/png;base64, ${file.base64}`}
              alt="first frame of gif"
              class="max-h-40 sm:max-h-60 md:max-h-80 lg:max-h-[32rem] w-auto"
            />
          {:else}
            <img
              src={getImageUrl(file.path)}
              alt="preview"
              class="max-h-40 sm:max-h-60 md:max-h-80 lg:max-h-[32rem] w-auto"
            />
          {/if}
        {:else}
          <p>no image</p>
        {/if}

        <p class="truncate w-full">
          {file.name}
        </p>
      </div>
    {/each}
  </div>
</section>

<style>
  /*   @container (min-width: 1200px) {
    section {
      grid-template-columns: repeat(5, 1fr);
      div {
        height: 15cqh;
        width: 15cqw;
      }
    }
  } */
  section {
    container-type: inline-size;
    container-name: main;
  }

  @container (max-width: 600px) {
    div {
      grid-template-columns: repeat(3, 1fr);
      .ffContainer {
        height: 20cqh;
        width: 25cqw;
      }
    }
  }
  /* pl-10 pr-10 */
  @container (max-width: 400px) {
    div {
      grid-template-columns: repeat(2, 1fr);
      padding-left: 0;
      padding-right: 0;
      .ffContainer {
        width: 30cqw;
      }
    }
  }

  @container (max-width: 270px) {
    div {
      grid-template-columns: 1fr;
      .ffContainer {
        width: 50cqw;
      }
    }
  }

  section::-webkit-scrollbar {
    width: 10px;
  }

  section::-webkit-scrollbar-track {
    background: #fff;
    border-radius: 10px;
  }

  section::-webkit-scrollbar-thumb {
    background: #888;
    border-radius: 10px;
  }

  section::-webkit-scrollbar-thumb:hover {
    background: #555;
  }
</style>

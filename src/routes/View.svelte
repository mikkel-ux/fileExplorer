<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { updateHistory, getCurrentPath } from "../stores/tabsStore";
  import { secsectFile, selectedFile } from "../stores/tabsStore";
  import { invoke } from "@tauri-apps/api/core";
  import { Folder, Inspect } from "@lucide/svelte";
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
    } catch (error) {
      console.log("error", error);
    }
  });

  const handleClick = (file: FileDataType) => {
    if (clickTimer) {
      clearTimeout(clickTimer);
      clickTimer = null;
      console.log("double click");
    } else {
      clickTimer = setTimeout(() => {
        console.log("single click");
        console.log($state.snapshot(file));
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
        console.log("double key click");
      } else {
        enterTimer = setTimeout(() => {
          console.log("single key click");
          secsectFile(file);

          enterTimer = null;
        }, 200);
      }
    }
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

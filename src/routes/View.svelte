<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { updateHistory, getCurrentPath } from "../stores/tabsStore";
  import { invoke } from "@tauri-apps/api/core";

  onMount(async () => {
    /* let test = await invoke("test_get_files_in_dirs", {
      path: "C:/Users/rumbo/.testFoulderForFE/test",
    });
    console.log("test", test);
  }); */

    try {
      let test = await invoke("get_files_dirs_in_dir", {
        path: "C:/Users/rumbo/.testFoulderForFE",
      });
      console.log("test", test);
    } catch (error) {
      console.log("error", error);
    }
  });
</script>

<section
  class="grid grid-cols-5 grid-rows-[repeat(1fr)] gap-4 justify-items-center text-center"
>
  <div class="bg-blue-950 w-20 h-20">box1</div>
  <div class="bg-blue-950 w-20 h-20">box2</div>
  <div class="bg-blue-950 w-20 h-20">box3</div>
  <div class="bg-blue-950 w-20 h-20">box4</div>
  <div class="bg-blue-950 w-20 h-20">box5</div>

  <div class="bg-blue-950 w-20 h-20">box1</div>
  <div class="bg-blue-950 w-20 h-20">box2</div>
  <div class="bg-blue-950 w-20 h-20">box3</div>
  <div class="bg-blue-950 w-20 h-20">box4</div>
  <div class="bg-blue-950 w-20 h-20">box5</div>
</section>

<style>
  @container (max-width: 500px) {
    /* h2 {
      color: red;
    } */

    section {
      grid-template-columns: 1fr 1fr 1fr;
    }
  }

  @container (max-width: 300px) {
    section {
      grid-template-columns: 1fr;
    }
  }
</style>

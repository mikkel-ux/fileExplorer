<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { addToHistory } from "../../stores/historyStore";

  const navigateToPage = () => {
    goto("/newPage");
    addToHistory("/newPage");
  };
</script>

<main class="h-full flex flex-col justify-center items-center text-black">
  <p>{page.url.pathname}</p>
  <button onclick={() => navigateToPage()}>go somewhere</button>
</main>

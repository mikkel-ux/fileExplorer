<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { addToHistory } from "../../stores/historyStore";

  const navigateToPage = () => {
    goto("/newPage");
    addToHistory("/newPage");
  };
</script>

<h2 class="text-3xl">slug test</h2>
<p>{page.url.pathname}</p>

<button onclick={() => navigateToPage()}>go somewhere</button>

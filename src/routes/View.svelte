<script lang="ts"></script>

<section class="test">
  <div>
    <h2>test</h2>
  </div>
</section>

<style>
  .test {
    container-type: inline-size;
    background-color: blue;
  }

  h2 {
    font-size: 5cqi;
  }

  @container (min-width: 400px) {
    h2 {
      color: black;
    }
  }
</style>

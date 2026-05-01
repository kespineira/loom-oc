<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import TextInput from "./TextInput.svelte";

  type Props = HTMLInputAttributes & {
    value?: string;
    invalid?: boolean;
  };

  let { value = $bindable(""), ...rest }: Props = $props();
</script>

<TextInput bind:value mono {...rest} />

List of types -> workbench -> new type definition
new type definition -> -> component


export interface Type {
  name: string;
  configuration: any;
}

class ConfigureRanking extends LitElement {}

export const ranking: Type = {
  name: 'Ranking',
  configure: ConfigureRanking,
  create:{
  imports: `
    import { Textfield } from '@material/mwc-textfield';
  `, 
  render: (configuration) =>
`<mwc-slider min=${configuration.min} max=${configuration.max}></mwc-slider>`,
    } ,
  detail: RankingDetail,
};


// App.vue

<template>
  <div class="column">
    <mwc-slider min="3" max="20" :value="ranking.value" @change="ranking = $event.target.value"></mwc-slider>
  </div>
</template>
<script>
import '@material/mwc-slider'
</script>


<template>
  <div class="column">
    <ranking min="3" max="20" :value="obj.ranking" @change="obj.ranking = $event.target.value"></ranking>
  </div>
</template>
<script>
import '@typecraft/ranking'
</script>
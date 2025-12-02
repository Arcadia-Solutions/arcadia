<template>
  <Tabs :value="Object.keys(orderedLinks)[0]" size="small">
    <TabList>
      <Tab v-for="(category, categoryName) in orderedLinks" :key="categoryName" :value="categoryName">{{ categoryName }}</Tab>
    </TabList>
    <TabPanels>
      <TabPanel v-for="(category, categoryName) in orderedLinks" :key="categoryName" :value="categoryName">
        <Tabs :value="Object.keys(category)[0]" size="small">
          <TabList>
            <Tab v-for="(link, linkName) in category" :key="linkName" :value="linkName">{{ linkName }}</Tab>
          </TabList>
          <TabPanels>
            <TabPanel v-for="(link, linkName) in category" :key="linkName" :value="linkName">
              <iframe style="width: 100%; aspect-ratio: 16 / 9; border: 0" :src="link" allowfullscreen />
            </TabPanel>
          </TabPanels>
        </Tabs>
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import { computed } from 'vue'
import type { EmbeddedLinks } from '@/services/api/titleGroupService'

const props = defineProps<{
  links: EmbeddedLinks
}>()

const orderedLinks = computed(() => {
  const newLinks: EmbeddedLinks = {}
  if (props.links.Trailers) {
    newLinks.Trailers = props.links.Trailers
  }
  for (const key in props.links) {
    if (key !== 'Trailers') {
      newLinks[key] = props.links[key]
    }
  }
  return newLinks
})
</script>

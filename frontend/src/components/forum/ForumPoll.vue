<template>
  <ContentContainer class="forum-poll" :container-title="poll.question">
    <template #top-left>
      <i class="pi pi-chart-bar poll-icon" />
    </template>
    <div v-if="showResults" class="poll-results">
      <div v-for="option in poll.options" :key="option.id" class="result-row">
        <div class="result-label">
          <span>{{ option.content }}</span>
          <span class="result-count">{{ t('forum.votes', { n: votesOf(option) }, votesOf(option)) }} ({{ percentOf(votesOf(option)) }}%)</span>
        </div>
        <ProgressBar :value="percentOf(votesOf(option))" :showValue="false" />
      </div>
      <div class="poll-footer">
        <span>
          {{ t('forum.votes', { n: totalVotes }, totalVotes) }}
          <template v-if="blankVotes > 0"> · {{ t('forum.blank_vote') }}: {{ blankVotes }}</template>
        </span>
        <span v-if="poll.has_voted" class="voted-badge"><i class="pi pi-check" /> {{ t('forum.you_voted') }}</span>
      </div>
    </div>

    <div v-else class="poll-vote">
      <div v-for="option in poll.options" :key="option.id" class="vote-row">
        <RadioButton v-model="selectedOptionId" :inputId="`poll-option-${option.id}`" :value="option.id" size="small" />
        <label :for="`poll-option-${option.id}`">{{ option.content }}</label>
      </div>
      <div class="vote-row">
        <RadioButton v-model="selectedOptionId" inputId="poll-option-blank" value="blank" size="small" />
        <label for="poll-option-blank">{{ t('forum.blank_vote') }}</label>
      </div>
      <Button
        class="vote-button"
        size="small"
        icon="pi pi-check"
        :label="t('forum.vote')"
        :loading="voting"
        :disabled="selectedOptionId === null"
        @click="vote"
      />
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Button, ProgressBar, RadioButton } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import { createForumPollVote, type ForumPollHierarchy, type ForumPollOptionResult } from '@/services/api-schema'

const { t } = useI18n()

const props = defineProps<{ poll: ForumPollHierarchy }>()

const poll = ref<ForumPollHierarchy>(props.poll)
const selectedOptionId = ref<number | 'blank' | null>(null)
const voting = ref(false)

const showResults = computed(() => poll.value.has_voted)
const blankVotes = computed(() => poll.value.blank_votes_amount ?? 0)
const votesOf = (option: ForumPollOptionResult) => option.votes_amount ?? 0
const totalVotes = computed(() => poll.value.options.reduce((sum, o) => sum + (o.votes_amount ?? 0), 0))
const percentOf = (votes: number) => (totalVotes.value === 0 ? 0 : Math.round((votes / totalVotes.value) * 100))

const vote = async () => {
  if (selectedOptionId.value === null) return
  voting.value = true
  createForumPollVote({
    forum_poll_id: poll.value.id,
    forum_poll_option_id: selectedOptionId.value === 'blank' ? null : selectedOptionId.value,
  })
    .then((updatedPoll) => {
      poll.value = updatedPoll
    })
    .finally(() => {
      voting.value = false
    })
}
</script>

<style scoped>
.forum-poll {
  margin-bottom: 20px;
}
.forum-poll :deep(.top > div:first-child) {
  display: flex;
  align-items: center;
  gap: 8px;
}
.forum-poll :deep(.container-title) {
  margin-bottom: 0;
}
.poll-icon {
  color: var(--color-primary);
}
.poll-vote {
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: flex-start;
}
.vote-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.vote-row label {
  cursor: pointer;
}
.vote-button {
  margin-top: 5px;
}
.poll-results {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.result-label {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
}
.result-count {
  color: var(--p-text-muted-color);
  font-size: 0.85em;
}
.poll-footer {
  display: flex;
  justify-content: space-between;
  margin-top: 5px;
  color: var(--p-text-muted-color);
  font-size: 0.85em;
}
.voted-badge {
  display: flex;
  align-items: center;
  gap: 5px;
}
</style>

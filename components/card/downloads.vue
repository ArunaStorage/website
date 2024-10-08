<script setup lang="ts">
import {IconCloudDown, IconCloudCancel, IconCloudCheck, IconCloudCog, IconCloudPause, IconCloudQuestion} from '@tabler/icons-vue';
import {type EndpointInfo, fetchEndpoint} from '~/composables/api_wrapper';
import { storagemodelsv2ReplicationStatus } from '../../composables/aruna_api_json';

const props = defineProps<{
  endpoints: EndpointInfo[] | undefined
}>();

const emit = defineEmits<{(e: 'download', endpointId: string): void}>();

const endpointData = {};
await Promise.all(props.endpoints.map(async (endpointInfo: EndpointInfo) => {
  try {
    endpointData[endpointInfo.id] = await fetchEndpoint(endpointInfo.id);
  } catch (error) {
    console.log(error.code);
    console.log(error.message);
  }
}));

function toReplicationStatusIcon(variant: storagemodelsv2ReplicationStatus | undefined): string {
    switch (variant) {
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_ERROR: return IconCloudCancel
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_FINISHED: return IconCloudCheck
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_RUNNING: return IconCloudCog
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_WAITING: return IconCloudPause
      default: return IconCloudQuestion
    }
}

function toReplicationStatusColor(variant: storagemodelsv2ReplicationStatus | undefined): string {
  switch (variant) {
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_ERROR: return "red"
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_FINISHED: return "green"
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_RUNNING: return "orange"
      case storagemodelsv2ReplicationStatus.REPLICATION_STATUS_WAITING: return "orange"
      default: return "orange"
    }
}

</script>

<template>
  <div class="-m-1.5 overflow-x-auto">
    <div class="p-1.5 min-w-full inline-block align-middle">
      <div class="overflow-hidden">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead>
          <tr>
            <th scope="col" class="px-6 py-3 text-start text-sm font-medium text-gray-500 uppercase">
              Endpoint Name
            </th>
            <th scope="col" class="px-6 py-3 text-start text-sm font-medium text-gray-500 uppercase">
              Replication Status
            </th>
            <th scope="col" class="px-6 py-3 text-center text-sm font-medium text-gray-500 uppercase">
              Download
            </th>
          </tr>
          </thead>

          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
          <tr v-for="endpoint in props.endpoints" class="hover:bg-gray-100 dark:hover:bg-gray-700">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-700 dark:text-gray-200">
                {{ endpointData[endpoint.id].name }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-700 dark:text-gray-200 flex item-center gap-2">
              <span class="">{{ toReplicationStatusStr(endpoint.status) }}</span>
              <component :is="toReplicationStatusIcon(endpoint.status)" class="flex-shrink-0" :color="toReplicationStatusColor(endpoint.status)"></component>
            </td>
            <td class="px-6 py-4 text-center whitespace-nowrap text-sm font-medium text-gray-700 dark:text-gray-200">
              <button
                  type="button"
                  title="Download Object"
                  @click="emit('download', endpoint.id)"
                  :disabled="endpoint.status != storagemodelsv2ReplicationStatus.REPLICATION_STATUS_FINISHED"
                  class="inline-flex grow justify-center font-semibold rounded-lg border border-transparent text-gray-600 dark:text-white hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none">
                  <IconCloudDown class="flex-shrink-0"/>
              </button>
            </td>
          </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
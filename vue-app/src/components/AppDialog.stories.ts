import type { Meta, StoryObj } from '@storybook/vue3-vite'

import AppDialog from './AppDialog.vue'
import DsButton from './DsButton.vue'

const meta = {
  title: 'Components/AppDialog',
  component: AppDialog,
  tags: ['autodocs'],
  parameters: {
    docs: {
      description: {
        component:
          'Reusable modal dialog with title, description, body, footer, and backdrop close behavior.',
      },
    },
  },
  argTypes: {
    open: { control: 'boolean' },
    title: { control: 'text' },
    description: { control: 'text' },
  },
  args: {
    open: true,
    title: 'Add to basket',
    description: 'Review the item before adding it to your basket.',
  },
} satisfies Meta<typeof AppDialog>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
  render: (args) => ({
    components: { AppDialog, DsButton },
    setup: () => ({ args }),
    template: `
      <AppDialog v-bind="args" v-model:open="args.open">
        <p>Organic bananas, one bunch.</p>
        <template #footer>
          <DsButton variant="white" @click="args.open = false">Cancel</DsButton>
          <DsButton @click="args.open = false">Add item</DsButton>
        </template>
      </AppDialog>
    `,
  }),
}

export const WithoutFooter: Story = {
  args: {
    title: 'Delivery information',
    description: 'Your order is delivered within two hours.',
  },
  render: (args) => ({
    components: { AppDialog },
    setup: () => ({ args }),
    template: `
      <AppDialog v-bind="args" v-model:open="args.open">
        <p>We will notify you when your order is on the way.</p>
      </AppDialog>
    `,
  }),
}

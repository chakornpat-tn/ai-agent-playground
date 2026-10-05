import type { Meta, StoryObj } from '@storybook/vue3-vite'

import QuantityControl from './QuantityControl.vue'

const meta = {
  title: 'Components/QuantityControl',
  component: QuantityControl,
  tags: ['autodocs'],
  parameters: {
    docs: {
      description: {
        component: 'Accessible quantity stepper that prevents values below one.',
      },
    },
  },
  argTypes: {
    modelValue: { control: { type: 'number', min: 1, step: 1 } },
  },
  args: {
    modelValue: 2,
  },
} satisfies Meta<typeof QuantityControl>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
  render: (args) => ({
    components: { QuantityControl },
    setup: () => ({ args }),
    template: '<QuantityControl v-bind="args" v-model="args.modelValue" />',
  }),
}

export const StartingAtOne: Story = {
  args: {
    modelValue: 1,
  },
  render: (args) => ({
    components: { QuantityControl },
    setup: () => ({ args }),
    template: '<QuantityControl v-bind="args" v-model="args.modelValue" />',
  }),
}

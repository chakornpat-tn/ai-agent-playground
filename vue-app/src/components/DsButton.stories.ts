import type { Meta, StoryObj } from '@storybook/vue3-vite'

import DsButton from './DsButton.vue'

const meta = {
  title: 'Components/DsButton',
  component: DsButton,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'Primary action button with filled, outlined, and white variants.',
      },
    },
  },
  argTypes: {
    variant: {
      control: 'select',
      options: ['primary', 'outline', 'white'],
    },
    block: {
      control: 'boolean',
    },
    type: {
      control: 'select',
      options: ['button', 'submit'],
    },
  },
  args: {
    variant: 'primary',
    block: false,
    type: 'button',
  },
} satisfies Meta<typeof DsButton>

export default meta

type Story = StoryObj<typeof meta>

const renderButton =
  (label: string): Story['render'] =>
  (args) => ({
    components: { DsButton },
    setup: () => ({ args, label }),
    template: '<DsButton v-bind="args">{{ label }}</DsButton>',
  })

export const Primary: Story = {
  render: renderButton('Add to cart'),
}

export const Outline: Story = {
  args: {
    variant: 'outline',
  },
  render: renderButton('View details'),
}

export const White: Story = {
  args: {
    variant: 'white',
  },
  render: renderButton('Save for later'),
}

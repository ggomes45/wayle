import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Wayle',
  description: 'A configurable desktop shell for Wayland.',
  cleanUrls: true,
  lastUpdated: true,

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/wayle.svg' }],
  ],

  markdown: {
    theme: {
      light: 'github-light',
      dark: 'tokyo-night',
    },
  },

  themeConfig: {
    logo: '/wayle.svg',

    nav: [
      { text: 'Guide', link: '/guide/getting-started', activeMatch: '^/guide/' },
      { text: 'Config', link: '/config/', activeMatch: '^/config/' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            {
              text: 'Getting started',
              link: '/guide/getting-started',
              collapsed: false,
              items: [
                { text: 'Arch Linux', link: '/guide/getting-started-arch' },
                { text: 'Debian / Ubuntu', link: '/guide/getting-started-debian' },
                { text: 'Fedora', link: '/guide/getting-started-fedora' },
                { text: 'NixOS', link: '/guide/getting-started-nixos' },
              ],
            },
            { text: 'Editing config', link: '/guide/editing-config' },
            { text: 'Bars and layouts', link: '/guide/bars-and-layouts' },
            { text: 'Themes', link: '/guide/themes' },
            { text: 'Custom styles', link: '/guide/custom-styles' },
            { text: 'Custom icons', link: '/guide/custom-icons' },
            { text: 'Custom modules', link: '/guide/custom-modules' },
            { text: 'CLI', link: '/guide/cli' },
          ],
        },
        {
          text: 'Also see',
          items: [
            { text: 'Config reference', link: '/config/' },
          ],
        },
      ],

      '/config/': [
        {
          text: 'Config',
          items: [
            { text: 'Overview', link: '/config/' },
            { text: 'Types', link: '/config/types' },
          ],
        },
        {
          text: 'Top-level',
          items: [
            { text: 'bar', link: '/config/bar' },
            { text: 'styling', link: '/config/styling' },
            { text: 'general', link: '/config/general' },
            { text: 'osd', link: '/config/osd' },
            { text: 'wallpaper', link: '/config/wallpaper' },
          ],
        },
        {
          text: 'Modules',
          items: [
            { text: 'battery', link: '/config/modules/battery' },
            { text: 'brightness', link: '/config/modules/brightness' },
            { text: 'clock', link: '/config/modules/clock' },
            { text: 'custom', link: '/config/modules/custom' },
            { text: 'dashboard', link: '/config/modules/dashboard' },
            { text: 'hyprland-workspaces', link: '/config/modules/hyprland-workspaces' },
            { text: 'keyboard-input', link: '/config/modules/keyboard-input' },
            { text: 'media', link: '/config/modules/media' },
            { text: 'microphone', link: '/config/modules/microphone' },
            { text: 'network', link: '/config/modules/network' },
            { text: 'notifications', link: '/config/modules/notifications' },
            { text: 'separator', link: '/config/modules/separator' },
            { text: 'systray', link: '/config/modules/systray' },
            { text: 'volume', link: '/config/modules/volume' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/wayle-rs/wayle' },
      { icon: 'discord', link: 'https://discord.gg/GYRGnNMf2c' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright 2026 Wayle contributors',
    },

    editLink: {
      pattern: 'https://github.com/wayle-rs/wayle/edit/master/docs/:path',
      text: 'Edit this page on GitHub',
    },

    outline: {
      level: [2, 3],
    },

    search: {
      provider: 'local',
    },
  },
})

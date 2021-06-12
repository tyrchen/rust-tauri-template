<template>
  <q-layout view="lHh lpR lFf" class="bg-grey-1">
    <q-header elevated class="bg-white text-grey-8" height-hint="64">
      <q-toolbar class="toolbar">
        <q-btn
          flat
          dense
          round
          @click="leftDrawerOpen = !leftDrawerOpen"
          aria-label="Menu"
          icon="menu"
          class="q-mr-sm"
        />

        <q-input
          class="toolbar-input"
          outlined
          dense
          v-model="search"
          color="bg-grey-7 shadow-1"
          placeholder="Search for topics, locations & sources"
        >
          <template v-slot:prepend>
            <q-icon v-if="search === ''" name="search" />
            <q-icon
              v-else
              name="clear"
              class="cursor-pointer"
              @click="search = ''"
            />
          </template>
          <template v-slot:append>
            <q-btn flat dense round aria-label="Menu" icon="arrow_drop_down">
              <q-menu anchor="bottom end" self="top end">
                <div class="q-pa-md" style="width: 400px">
                  <div class="text-body2 text-grey q-mb-md">
                    Narrow your search results
                  </div>

                  <div class="row items-center">
                    <div class="col-3 text-subtitle2 text-grey">
                      Exact phrase
                    </div>
                    <div class="col-9 q-pl-md">
                      <q-input dense v-model="exactPhrase" />
                    </div>

                    <div class="col-3 text-subtitle2 text-grey">Has words</div>
                    <div class="col-9 q-pl-md">
                      <q-input dense v-model="hasWords" />
                    </div>

                    <div class="col-3 text-subtitle2 text-grey">
                      Exclude words
                    </div>
                    <div class="col-9 q-pl-md">
                      <q-input dense v-model="excludeWords" />
                    </div>

                    <div class="col-3 text-subtitle2 text-grey">Website</div>
                    <div class="col-9 q-pl-md">
                      <q-input dense v-model="byWebsite" />
                    </div>

                    <div class="col-12 q-pt-lg row justify-end">
                      <q-btn
                        flat
                        dense
                        no-caps
                        color="grey-7"
                        size="md"
                        style="min-width: 68px"
                        label="Search"
                        v-close-popup
                      />
                      <q-btn
                        flat
                        dense
                        no-caps
                        color="grey-7"
                        size="md"
                        style="min-width: 68px"
                        @click="onClear"
                        label="Clear"
                        v-close-popup
                      />
                    </div>
                  </div>
                </div>
              </q-menu>
            </q-btn>
          </template>
        </q-input>

        <q-space />

        <div class="q-gutter-sm row items-center no-wrap">
          <q-btn round dense flat color="text-grey-7" icon="apps">
            <q-tooltip>Google Apps</q-tooltip>
          </q-btn>
          <q-btn round dense flat color="grey-8" icon="notifications">
            <q-badge color="red" text-color="white" floating> 2 </q-badge>
            <q-tooltip>Notifications</q-tooltip>
          </q-btn>
          <q-btn round flat>
            <q-avatar size="26px">
              <img src="https://cdn.quasar.dev/img/boy-avatar.png" />
            </q-avatar>
            <q-tooltip>Account</q-tooltip>
          </q-btn>
        </div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
      content-class="bg-white"
      :width="280"
    >
      <q-toolbar>
        <q-toolbar-title shrink class="row items-center no-wrap">
          <img
            width="36"
            src="https://github.githubassets.com/images/modules/profile/badge--acv-64.png"
          />
          <span class="q-ml-sm">Example</span>
        </q-toolbar-title>
      </q-toolbar>
      <q-scroll-area class="fit">
        <q-list padding class="text-grey-8">
          <q-item
            class="drawer-item"
            v-ripple
            v-for="link in links1"
            :key="link.text"
            clickable
          >
            <q-item-section avatar>
              <q-icon :name="link.icon" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ link.text }}</q-item-label>
            </q-item-section>
          </q-item>

          <q-separator inset class="q-my-sm" />
          <q-expansion-item
            expand-separator
            icon="assignment"
            label="Spaces"
            caption="1 spaces"
            default-opened
          >
            <q-item
              class="drawer-item"
              v-ripple
              v-for="space in spaces"
              :key="space.text"
              clickable
            >
              <q-item-section avatar>
                <q-icon :name="space.icon" size="xs" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ space.text }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-expansion-item>

          <q-expansion-item
            expand-separator
            icon="dashboard"
            label="Dashboards"
            caption="1 dashboards"
            default-opened
          >
            <q-item
              class="drawer-item"
              v-ripple
              v-for="board in dashboards"
              :key="board.text"
              clickable
            >
              <q-item-section avatar>
                <q-icon :name="board.icon" size="xs" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ board.text }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-expansion-item>

          <q-separator inset class="q-my-sm" />
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { fasGlobeAmericas, fasFlask } from "@quasar/extras/fontawesome-v5";
export default {
  name: "MayLayout",
  data() {
    return {
      leftDrawerOpen: false,
      search: "",
      showAdvanced: false,
      showDateOptions: false,
      exactPhrase: "",
      hasWords: "",
      excludeWords: "",
      byWebsite: "",
      byDate: "Any time",
      links1: [
        { icon: "home", text: "Home" },
        { icon: "record_voice_over", text: "Notifications" },
        { icon: "star_border", text: "Favourites" },
        { icon: "filter_alt", text: "Saved filters" },
        { icon: "account_circle", text: "User settings" },
      ],
      spaces: [{ icon: "list_alt", text: "Default space" }],

      dashboards: [{ icon: "business", text: "Default dashboard" }],
    };
  },
  methods: {
    onClear() {
      this.exactPhrase = "";
      this.hasWords = "";
      this.excludeWords = "";
      this.byWebsite = "";
      this.byDate = "Any time";
    },
    changeDate(option) {
      this.byDate = option;
      this.showDateOptions = false;
    },
  },
};
</script>

<style lang="sass">
.toolbar
  height: 64px
.toolbar-input
    width: 55%
.drawer-item
    line-height: 24px
    border-radius: 0 24px 24px 0
    margin-right: 12px
    .q-item__section--avatar
      .q-icon
        color: #5f6368
    .q-item__label
      color: #3c4043
      letter-spacing: .01785714em
      font-size: .875rem
      font-weight: 500
      line-height: 1.25rem
.drawer-footer-link
    color: inherit
    text-decoration: none
    font-weight: 500
    font-size: .75rem
    &:hover
      color: #000
</style>

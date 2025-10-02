import { createRouter, createWebHashHistory, Router } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'root',
      redirect: '/home',
      children: [
        {
          path: 'home',
          name: 'Home',
          component: () => import('../views/Home.vue'),
          children: [
            {
              path: 'json',
              name: 'json',
              component: () => import('../views/JsonTool.vue')
            },
            {
              path: 'date',
              name: 'date',
              component: () => import('../views/DateTool.vue'),
              children: [
                {
                  path: 'timestamp',
                  name: 'timestamp',
                  component: () => import('../views/Timestamp.vue')
                },
                {
                  path: 'calculator',
                  name: 'dateCalculator',
                  component: () => import('../views/DateCalculator.vue')
                }
              ]
            },
            {
              path: 'hash',
              name: 'hash',
              component: () => import('../views/HashTool.vue'),
              children: [
                {
                  path: 'calc',
                  name: 'hashCalc',
                  component: () => import('../views/HashCalc.vue')
                },
                {
                  path: 'compare',
                  name: 'hashCompare',
                  component: () => import('../views/HashCompare.vue')
                }
              ]
            },
            {
              path: 'gen',
              name: 'generator',
              component: () => import('../views/GeneratorTool.vue'),
              children: [
                {
                  path: 'randomCh',
                  name: 'randomCharacter',
                  component: () => import('../views/GenerateCharacter.vue')
                },
                {
                  path: 'uuid',
                  name: 'uuid',
                  component: () => import('../views/GenerateUUID.vue')
                }
              ]
            },
            {
              path: 'diff',
              name: 'diff',
              component: () => import('../views/TextDiff.vue')
            }
          ]
        },
        {
          path: 'config',
          name: 'config',
          component: () => import('../views/Config.vue')
        }
      ]
    }
  ]
}) as Router

export default router

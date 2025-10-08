import {createRouter, createWebHashHistory, Router} from 'vue-router'

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
                            path: 'encrypt',
                            name: 'encrypt',
                            component: () => import('../views/EncryptTool.vue'),
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
                                },
                                {
                                    path: 'base64',
                                    name: 'base64Tool',
                                    component: () => import('../views/Base64Tool.vue'),
                                    children: [
                                        {
                                            path: 'encrypt',
                                            name: 'base64Encrypt',
                                            component: () => import('../views/Base64Encrypt.vue')
                                        },
                                        {
                                            path: 'decrypt',
                                            name: 'base64Decrypt',
                                            component: () => import('../views/Base64Decrypt.vue')
                                        }
                                    ]
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
                    component: () => import('../views/Config.vue'),
                    children: [
                        {
                            path: 'system',
                            name: 'system',
                            component: () => import('../views/config/System.vue')
                        },
                        {
                            path: 'appearance',
                            name: 'appearance',
                            component: () => import('../views/config/Appearance.vue')
                        },
                        {
                            path: 'about',
                            name: 'about',
                            component: () => import('../views/config/About.vue')
                        }
                    ]
                },
                {
                    path: 'update',
                    name: 'update',
                    component: () => import('../views/Update.vue')
                }
            ]
        }
    ]
}) as Router

export default router

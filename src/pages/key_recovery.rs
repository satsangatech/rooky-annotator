use nostr_minions::{
    browser_api::IdbStoreManager,
    key_manager::{NostrIdAction, NostrIdStore},
};
use shady_minions::ui::{Button, Card, CardContent, CardHeader, CardTitle};
use yew::prelude::*;

#[function_component(KeyRecoveryPage)]
pub fn key_recovery_page() -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();

    html! {
        <Card class="max-w-sm h-fit">
            <CardHeader>
                <CardTitle>
                    <div class="flex items-center space-x-3">
                        <yew_router::components::Link<crate::router::AnnotatorRoute>
                            to={crate::router::AnnotatorRoute::Home}>
                            <Button
                                variant={shady_minions::ui::ButtonVariant::Outline}
                                size={shady_minions::ui::ButtonSize::Small}
                                >
                                <lucide_yew::ArrowLeft class="size-4" />
                            </Button>
                        </yew_router::components::Link<crate::router::AnnotatorRoute>>
                        <div class="flex items-center space-x-3 pb-2">
                            <lucide_yew::Key class="text-primary size-6" />
                            <h3 class="text-2xl font-bold">{ language_ctx.t("key_recovery_title") }</h3>
                        </div>
                    </div>
                </CardTitle>
            </CardHeader>
            <CardContent class="space-y-6">
                <div class="flex gap-3">
                    <lucide_yew::TriangleAlert class="text-yellow-500 size-5 mt-1 flex-shrink-0" />
                    <p class="text-muted">
                        { language_ctx.t("key_recovery_use_key") }
                        <br />
                        { language_ctx.t("key_recovery_keep_safe") }
                    </p>
                </div>
                <KeyRecoverySection />
            </CardContent>
        </Card>
    }
}

#[function_component(KeyRecoverySection)]
fn key_recovery_section() -> Html {
    let key_ctx = use_context::<NostrIdStore>().expect("No NostrIdStore found");
    let language_ctx = crate::contexts::language::use_language_ctx();

    let priv_key_copied = use_state(|| false);
    let mnemonic_copied = use_state(|| false);
    let pubkey_copied = use_state(|| false);
    // let npub_copied = use_state(|| false);

    // State for showing/hiding sensitive data
    let show_sensitive = use_state(|| false);

    let pubkey = use_state(String::new);

    // State for private key and recovery phrase
    let priv_key = use_state(|| "".to_string());
    let recovery_phrase = use_state(Vec::<String>::new);
    let id_state = use_state(|| key_ctx.get_identity().cloned());

    // Check if user is using extension-based identity
    let is_extension = use_state(|| false);

    // Fetch key information and determine if user is using extension
    let priv_key_handle = priv_key.clone();
    let recovery_phrase_handle = recovery_phrase.clone();
    // let is_extension_handle = is_extension.clone();
    let id_handle = id_state.clone();
    let pubkey_setter = pubkey.setter();
    let notification_text = language_ctx.t("notification_copied_to_clipboard");
    use_effect_with(key_ctx.clone(), move |key_handle| {
        let priv_key_handle = priv_key_handle.clone();
        let recovery_phrase_handle = recovery_phrase_handle.clone();
        // let is_extension_handle = is_extension_handle.clone();
        let key_handle = key_handle.clone();

        let pubkey_setter = pubkey_setter.clone();
        yew::platform::spawn_local(async move {
            // Check if identity is extension-based by attempting to get the key
            id_handle.set(key_handle.get_identity().cloned());

            if let Some(mut key) = key_handle.get_nostr_key().await {
                // Set key as extractable to access sensitive data
                key.set_extractable(true);

                if let Ok(npub) = key.npub() {
                    pubkey_setter.set(npub.to_string());
                } else {
                    web_sys::console::log_1(&"Failed to retrieve public key".into());
                };

                // Get private key as hex
                let Ok(secret_key) = key.nsec() else {
                    web_sys::console::log_1(&"Failed to retrieve private key".into());
                    return;
                };
                priv_key_handle.set(secret_key);
                // Get the recovery phrase (mnemonic)
                // The key must be extractable to access the mnemonic
                if let Ok(mnemonic) = key.mnemonic(nostr_minions::nostro2_signer::Language::English)
                {
                    let words: Vec<String> =
                        mnemonic.split_whitespace().map(String::from).collect();
                    recovery_phrase_handle.set(words);
                }
            }
        });
        || {}
    });

    let onclick_copy_privkey = {
        let secret_key = priv_key.clone();
        let copied = priv_key_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&secret_key);
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let onclick_copy_phrase = {
        let phrase = recovery_phrase.clone();
        let copied = mnemonic_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&phrase.join(" "));
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let onclick_copy_pubkey = {
        let pubkey = pubkey.clone();
        let copied = pubkey_copied.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&pubkey);
            copied.set(true);
            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            })
            .forget();
        })
    };

    let delete_key = {
        let key_handle = key_ctx.dispatcher();
        let id_state = id_state.clone();
        let lang_ctx = language_ctx.clone();
        Callback::from(move |_| {
            // if let Some(Ok(confirmed)) =
            if let Some(true) = web_sys::window().and_then(|win| {
                win.confirm_with_message(&lang_ctx.t("key_recovery_delete_confirm"))
                    .ok()
            }) {
                // First dispatch the DeleteIdentity action
                key_handle.dispatch(NostrIdAction::DeleteIdentity);
                let id = (*id_state).clone();
                yew::platform::spawn_local(async move {
                    let Some(id) = id else {
                        web_sys::console::log_1(&"No identity found to delete".into());
                        return;
                    };
                    if let Err(e) = id.delete_from_store().await {
                        web_sys::console::log_1(
                            &format!("Failed to delete identity: {:?}", e).into(),
                        );
                    }
                });
            }
        })
    };

    html! {
        <div class="space-y-6 overflow-y-auto pb-6">
            // Delete key button
            <div class="flex gap-3 w-full max-w-xs">
                // Show/Hide Toggle Button
                {
                    if !(*is_extension) {
                        html! {
                            <Button
                                onclick={
                                    let show_sensitive = show_sensitive.clone();
                                    Callback::from(move |_| show_sensitive.set(!*show_sensitive))
                                }
                                class="flex items-center gap-2 flex-1"
                            >
                                {
                                    if *show_sensitive {
                                        html! {
                                            <>
                                                <lucide_yew::EyeOff class="w-4 h-4" />
                                                <span>{ language_ctx.t("key_recovery_hide_data") }</span>
                                            </>
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <lucide_yew::Eye class="w-4 h-4" />
                                                <span>{ language_ctx.t("key_recovery_show_data") }</span>
                                            </>
                                        }
                                    }
                                }
                            </Button>
                        }
                    } else {
                        html! {}
                    }
                }
                <Button
                    onclick={delete_key}
                    variant={shady_minions::ui::ButtonVariant::Destructive}
                    class="flex items-center gap-2 flex-1"
                >
                    <span>{ language_ctx.t("key_recovery_delete") }</span>
                    <lucide_yew::Trash2 class="w-4 h-4" />
                </Button>
            </div>

            <div class="space-y-6">
                // Public Key Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_public_key") }</h3>
                    <div class="bg-muted p-4 rounded-lg flex gap-3">
                        <pre class="max-w-x16 truncate text-sm text-muted-foreground select-all">
                            {if pubkey.is_empty() {
                                "Loading..."
                            } else {
                                // format!("{}...{}", &pubkey[0..8], &pubkey[pubkey.len() - 8..])
                                // &*pubkey
                                if *pubkey_copied {
                                    &notification_text
                                } else {
                                    &*pubkey
                                }
                            }}
                        </pre>
                        {if !*pubkey_copied {
                            html! {
                                <button
                                    onclick={onclick_copy_pubkey}
                                    class="hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                    title={ language_ctx.t("key_recovery_copy_public_key") }
                                >
                                    <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                </button>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Private Key Section with warning
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_private_key") }</h3>

                    {
                        // if *is_extension {
                        //     html! {
                        //         <div class="bg-muted p-4 rounded-lg">
                        //             <div class="flex items-center text-gray-700 space-x-2">
                        //                 <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                        //                 // <p>{ language_ctx.t("key_recovery_no_private_key") }</p>
                        //             </div>
                        //         </div>
                        //     }
                        // } else
                        if *show_sensitive {
                            html! {
                                <div class="bg-muted p-4 rounded-lg overflow-x-auto flex gap-3">
                                    // <pre class="text-sm text-gray-800 whitespace-pre-wrap break-all select-all">
                                    <pre class="text-sm text-muted-foreground truncate select-all ">
                                        // {&*priv_key}
                                        {if priv_key.is_empty() {
                                            "Loading..."
                                        } else if *priv_key_copied {
                                            &notification_text
                                        } else {
                                            &*priv_key
                                        }}
                                    </pre>
                                    {if !*priv_key_copied {
                                        html! {
                                            <button
                                                onclick={onclick_copy_privkey}
                                                class="hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                                title={ language_ctx.t("key_recovery_copy_private_key") }
                                            >
                                                <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="bg-muted p-4 rounded-lg">
                                    <div class="text-muted-foreground italic">
                                        { language_ctx.t("key_recovery_hidden") }
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>
                // Recovery Phrase Section
                <div class="space-y-2">
                    <h3 class="text-lg font-medium text-muted">{ language_ctx.t("key_recovery_recovery_phrase") }</h3>
                    {
                        // if *is_extension {
                        //     html! {
                        //         <div class="bg-muted p-4 rounded-lg">
                        //             <div class="flex items-center text-gray-700 space-x-2">
                        //                 <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                        //     //            <p>{ language_ctx.t("key_recovery_extension_warning") }</p>
                        //             </div>
                        //         </div>
                        //     }
                        // } else
                        if *show_sensitive {
                            if recovery_phrase.is_empty() {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg">
                                        <div class="flex items-center text-muted-foreground space-x-2">
                                            <lucide_yew::TriangleAlert class="text-amber-500 w-5 h-5 flex-shrink-0" />
                                            <p>{ language_ctx.t("key_recovery_no_phrase") }</p>
                                        </div>
                                    </div>
                                }
                            } else if *mnemonic_copied {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg">
                                        <div class="text-muted-foreground italic">
                                            { language_ctx.t("notification_copied_to_clipboard") }
                                        </div>
                                    </div>
                                }
                            }
                            else {
                                html! {
                                    <div class="bg-muted p-4 rounded-lg relative text-xs">
                                        <div class="grid grid-cols-2 md:grid-cols-3 gap-2 pr-5 pb-2 max-h-24 overflow-y-auto">
                                            {
                                                recovery_phrase.iter().enumerate().map(|(i, word)| {
                                                    html! {
                                                        <div class="flex text-muted-foreground items-center">
                                                            <span class="w-6 text-right mr-2">{format!("{}.", i + 1)}</span>
                                                            <span class="font-mono bg-white px-2 py-1 rounded flex-grow">{word}</span>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                        <button
                                            onclick={onclick_copy_phrase}
                                            class="absolute top-2 right-2 p-2 hover:bg-muted hover:text-primary rounded-lg transition-colors"
                                            title={ language_ctx.t("key_recovery_copy_recovery_phrase") }
                                        >
                                            <lucide_yew::Copy class="w-5 h-5 text-muted-foreground" />
                                        </button>
                                    </div>
                                }
                            }
                        } else {
                            html! {
                                <div class="bg-muted p-4 rounded-lg">
                                    <div class="text-muted-foreground italic">
                                        { language_ctx.t("key_recovery_hidden") }
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>


            </div>
        </div>
    }
}

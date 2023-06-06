zsh_plugin_dir =  ${HOME}/.oh-my-zsh/custom/plugins/mono-repo

install_zsh_plugin:
	cargo build --release
	cp -r templates/mono-repo ${zsh_plugin_dir}
	cp target/release/mono-repo ${zsh_plugin_dir}/bin/mono-repo
	cargo clean

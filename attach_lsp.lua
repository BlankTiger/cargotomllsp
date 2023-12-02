print("a")
vim.lsp.start({
	name = "cargotomllsp",
	cmd = { "cargotomllsp" },
	root_dir = vim.fs.dirname(vim.fs.find({ 'Cargo.toml' }, { upward = true })[1]),
})

vim.api.nvim_create_autocmd('LspAttach', {
	callback = function(args)
		print("hello lsp")
		print(vim.inspect(args))
		-- vim.keymap.set('n', 'K', vim.lsp.buf.hover, { buffer = args.buf })
	end,
})

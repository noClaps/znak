export function escapeHTML(input: string) {
	return input
		.replaceAll(`&`, `&amp;`)
		.replaceAll(`"`, `&quot;`)
		.replaceAll(`'`, `&#x27;`)
		.replaceAll(`<`, `&lt;`)
		.replaceAll(`>`, `&gt;`);
}

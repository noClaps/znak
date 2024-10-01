export function escapeHTML(input: string) {
  return input.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

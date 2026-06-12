export function closeOtherProductShellMenus(current: HTMLDetailsElement) {
  const root = current.closest("[data-product-shell-menu-root]") || document;
  const menus = root.querySelectorAll<HTMLDetailsElement>("details[data-product-shell-menu]");

  menus.forEach((menu) => {
    if (menu !== current && !menu.contains(current) && !current.contains(menu)) {
      menu.open = false;
    }
  });
}

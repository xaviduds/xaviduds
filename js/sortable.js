import Sortable from "https://cdn.jsdelivr.net/npm/sortablejs@1.15.3/+esm";

document.querySelectorAll(".column.start").forEach((el, i) => {
  new Sortable(el, {
    animation: 175,
    ghostClass: "opacity-25",
    onEnd: () => {
      el.dispatchEvent(
        new CustomEvent("reordered", {
          detail: { orderInfo: [...el.children].map((e) => e.dataset.id) },
        }),
      );
    },
  });
});

document.querySelectorAll(".row.middle_y.mini_eca .row").forEach((row) => {
  new Sortable(row, {
    animation: 150,
    ghostClass: "opacity-25",
    direction: "horizontal",
    group: "tech-items", // optional: allow cross-row drag
    onEnd: () => {
      row.dispatchEvent(
        new CustomEvent("techReordered", {
          detail: {
            order: [...row.children].map((e) => e.dataset.id),
          },
        }),
      );
    },
  });
});

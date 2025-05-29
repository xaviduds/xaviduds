import Sortable from "https://cdn.jsdelivr.net/npm/sortablejs@1.15.3/+esm";

const mainContainer = document.getElementById("main");

new Sortable(mainContainer, {
  animation: 150,
  ghostClass: "opacity-0",
  direction: "horizontal",
  onEnd: (evt) => {
    mainContainer.dispatchEvent(
      new CustomEvent("reordered", {
        detail: { orderMain: [...mainContainer.children].map((e) => e.id) },
      }),
    );
  },
});

const projects = document.getElementById("projects");

new Sortable(projects, {
  animation: 150,
  ghostClass: "opacity-0",
  direction: "horizontal",
  onEnd: (evt) => {
    projects.dispatchEvent(
      new CustomEvent("reordered", {
        detail: { orderMain: [...projects.children].map((e) => e.id) },
      }),
    );
  },
});

function shuffleTwoRandomChildren(container) {
  if (!container) return;
  const children = [...container.children].filter(
    (c) => c instanceof HTMLElement,
  );
  if (children.length < 2) return;

  const [i, j] = getTwoRandomIndices(children.length);
  if (i === j) return;

  const a = children[i];
  const b = children[j];

  a.classList.add("glow");
  b.classList.add("glow");

  setTimeout(() => {
    a.classList.add("fade", "fade-out");
    b.classList.add("fade", "fade-out");

    setTimeout(() => {
      const nextSibling = a.nextSibling === b ? a : b;
      container.insertBefore(b, a);
      container.insertBefore(a, nextSibling);

      a.classList.remove("fade-out");
      b.classList.remove("fade-out");
      a.classList.add("fade-in");
      b.classList.add("fade-in");

      setTimeout(() => {
        a.classList.remove("glow", "fade", "fade-in");
        b.classList.remove("glow", "fade", "fade-in");

        container.dispatchEvent(
          new CustomEvent("reordered", {
            detail: { order: [...container.children].map((_, i) => i) },
          }),
        );
      }, 500);
    }, 300);
  }, 1200);
}

function getTwoRandomIndices(len) {
  const i = Math.floor(Math.random() * len);
  let j;
  do {
    j = Math.floor(Math.random() * len);
  } while (j === i);
  return [i, j];
}

function getRandomElement(selector) {
  const all = document.querySelectorAll(selector);
  return all.length ? all[Math.floor(Math.random() * all.length)] : null;
}

setInterval(() => {
  const mode = Math.floor(Math.random() * 3);

  if (mode === 0) {
    const main = document.querySelector("#main");
    shuffleTwoRandomChildren(main);
  } else if (mode === 1) {
    const projects = document.querySelector("#projects");
    if (!projects) return;
    const projectRows = [...projects.children].filter((el) =>
      el.classList.contains("project"),
    );
    if (projectRows.length > 1) shuffleTwoRandomChildren(projects);
  } else {
    const areaColumns = document.querySelectorAll(".column.start");
    const column = areaColumns[Math.floor(Math.random() * areaColumns.length)];

    if (column) {
      shuffleTwoRandomChildren(column);
    }

    const techRows = document.querySelectorAll(".row.middle_y.mini_eca .row");
    const techRow = techRows[Math.floor(Math.random() * techRows.length)];
    if (techRow) {
      shuffleTwoRandomChildren(techRow);
    }
  }
}, 10000);

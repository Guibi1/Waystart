use gpui::actions;

actions!(
    waystart,
    [
        SelectPrevEntry,
        SelectNextEntry,
        ExecuteEntry,
        ToggleFavorite,
        Close
    ]
);

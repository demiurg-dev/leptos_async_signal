# Sample crumbs

A sample Leptos application that showcases the use of an async signal. The main idea of the 
application is that breadcrumbs are displayed at the top level, but they are updated for each route
(e.g., home, post entry). The async signal enables breadcrumbs to be generated asynchronously in SSR 
mode on the server side, even though their content is defined deep within the application.

Relevant files:
- [app.rs](src/app.rs) - The main application logic. Documents each step.
- [db.rs](src/app.rs) - Mocks database of posts.
- [model.rs](src/model.rs) - A model of database posts.

The rest of files are defaults from Axum template.
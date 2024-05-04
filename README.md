[![Dioxus Spline Hero Image](https://raw.githubusercontent.com/splinetool/react-spline/main/.github/screenshots/hero.png)](https://my.spline.design/splinereactlogocopycopy-eaa074bf6b2cc82d870c96e262a625ae/)

# dioxus-spline

**dioxus-spline** allows you to export and use Spline scenes directly in your Dioxus websites.

🌈 [Spline](https://spline.design/) is a friendly 3d collaborative design tool for the web.

[Website](https://spline.design/) &mdash;
[Twitter](https://twitter.com/splinetool) &mdash;
[Community](https://discord.gg/M9hNDMqvnw) &mdash;
[Documentation](https://docs.spline.design/)

## Usage

### Basic:


To use dioxus-spline, first you have to go to the Spline editor, click on the **Export** button, select "**Code**".

You should see something like this:

<img width="250" src="https://raw.githubusercontent.com/splinetool/react-spline/main/.github/screenshots/react-export-pane.png">

You can copy the URL and pass it to the `Spline` component in Dioxus:

```rust
use dioxus-spline::Spline

Spline {
  scene: String::from("https://prod.spline.design/6Wq1Q7YGyM-iab9i/scene.splinecode") 
}
```

You should be able to see the scene you exported in your Dioxus app.


### Advanced

You can also:
1. Query Spline objects via `SplineApplication.find_object_by_name` or `SplineApplication.find_object_by_id`.
2. Attach event listeners to any `SplineEvent`
3. Trigger Spline Events and modify Spline objects!

To see how, visit examples and gh-page

Some additional helpful info can be found in the documentation:

https://docs.rs/dioxus-spline/latest/dioxus_spline/

## Acknowledgements

- Thanks to the Dioxus community, join their [discord](https://discord.gg/D6qNPSPn)!
- Thanks to the contributors to [react-spline](https://github.com/splinetool/react-spline)

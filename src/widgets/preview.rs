
mod imp {
    use adw::subclass::prelude::*;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Label, ScrolledWindow};

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = "
        <interface>
          <template class=\"Preview\" parent=\"AdwBin\">
            <child>
              <object class=\"GtkScrolledWindow\" id=\"scrolled_window\">
                <property name=\"child\">
                  <object class=\"Label\" id=\"label\">
                    <property name=\"vexpand\">true</property>
                    <property name=\"hexpand\">true</property>
                    <property name=\"xalign\">0</property>
                    <property name=\"yalign\">0</property>
                    <property name=\"wrap\">true</property>
                  </object>
                </property>
              </object>
            </property>
          </template>
        </interface>
    ")]
    pub struct Preview {
        #[template_child]
        pub(super) label: TemplateChild<Label>,
        #[template_child]
        pub(super) scrolled_window: TemplateChild<ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Preview {
        const NAME: &'static str = "Preview";
        type Type = super::Preview;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Preview {}
    impl WidgetImpl for Preview {}
    impl BinImpl for Preview {}
}

use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Preview(ObjectSubclass<imp::Preview>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Preview {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn render(&self, markdown: &str) {
        let markup = markdown_to_pango(markdown);
        self.imp().label.set_markup(&markup);
    }

    pub fn get_vadjustment(&self) -> gtk::Adjustment {
        self.imp().scrolled_window.vadjustment()
    }
}

fn markdown_to_pango(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut pango_output = String::new();
    let mut list_level = 0;
    for event in parser {
        match event {
            pulldown_cmark::Event::Text(text) => pango_output.push_str(&glib::markup_escape_text(&text)),
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Paragraph => pango_output.push_str("\n"),
                pulldown_cmark::Tag::Heading { level, .. } => pango_output.push_str(&format!("\n<span size='{}em' font_weight='bold'>", 2.0 / (level as u32 as f32))),
                pulldown_cmark::Tag::Emphasis => pango_output.push_str("<i>"),
                pulldown_cmark::Tag::Strong => pango_output.push_str("<b>"),
                pulldown_cmark::Tag::BlockQuote(_) => pango_output.push_str("\n<span foreground='gray'>"),
                pulldown_cmark::Tag::CodeBlock(_) => pango_output.push_str("\n<tt>"),
                pulldown_cmark::Tag::List(_) => {
                    list_level += 1;
                    pango_output.push_str("\n");
                }
                pulldown_cmark::Tag::Item => pango_output.push_str(&format!("{}* ", "  ".repeat(list_level - 1))),
                _ => {}
            },
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Heading(..) => pango_output.push_str("</span>\n"),
                pulldown_cmark::TagEnd::Emphasis => pango_output.push_str("</i>"),
                pulldown_cmark::TagEnd::Strong => pango_output.push_str("</b>"),
                pulldown_cmark::TagEnd::BlockQuote => pango_output.push_str("</span>\n"),
                pulldown_cmark::TagEnd::CodeBlock => pango_output.push_str("</tt>\n"),
                pulldown_cmark::TagEnd::List(_) => list_level -= 1,
                _ => {}
            },
            _ => {}
        }
    }
    pango_output
}

when prompting for suggestions in a rust file, the imports may be using

use whatever_types::prelude::*;

it should be a farily simple operation to materialize a list of all the types that are now present.
Language server integration with the prompting mechanism.
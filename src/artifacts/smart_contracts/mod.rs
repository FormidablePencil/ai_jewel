use std::collections::HashMap;
use smart_contract::payload::Parameters;

struct Post {
    post_pk: u16,
    // mod_privileges_pk:
}

struct Comment {
    comment_pk: u16,
    author_address: [u8; 32],
}

// struct PostEntry {
//     post_pk: u16,
// }

struct PostDB {
    // u16 = post_pk
    posts: HashMap<u16, Post>,
    // u16 = comment_pk
    posts_comments: HashMap<u16, Vec<Comment>>,
}

//todo- rename
#[smart_contract]
impl PostDB {
    fn init(_params: &mut Parameters) -> Self {
        Self {
            posts: HashMap::new(),
            posts_comments: HashMap::new(),
        }
    }

    fn save_post(&mut self, params: &mut Parameters) -> Result<(), String> {
        let post = Post {
            // author_address: params.sender,
            post_pk: params.read(),
        };

        // todo - test if post under the param is

        // self.posts.insert(entry.post_pk, post);
        // Ok(())
        todo!()
    }

    fn get_post(&mut self, params: &mut Parameters) -> Result<&Post, String> {
        let post_rk = params.read();
        let post = self.posts.get(post_rk);

        match post {
            None => return Err(format!("Found no post under reference key of {post_rk}.")),
            Some(p) => Ok(p),
        }
    }

    fn save_comment(&mut self, params: &mut Parameters) -> Result<(), String> {
        let post_rk = params.read();

        match self.posts.get(post_rk) {
            None => return Err(format!("")), //todo- better error responses
            Some(_) => {
                //todo- check if comments under post was created then create it if not
                match self.posts_comments.get(post_rk).is_some() {
                    None => {
                        //todo- check if comment under comment_pk exists.
                        let mut posts_comments = self.posts_comments.get(post_rk);
                        match posts_comments {
                            None => {}
                            Some(c) => {
                                // let mut comments:Vec<Comment> = *c;
                                // self.posts_comments.insert(post_rk, comments);
                            }
                        }
                        // self.posts_comments
                    },
                    Some(_) => {}
                }


                Ok(())
            },
        }
        // check that post_pk to submit to exists
        //
    }
}

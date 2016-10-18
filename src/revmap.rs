use std::collections::HashMap;
use std::collections::HashSet;

type RevMapMappedSet = HashMap<String, HashSet<String>>;

pub struct RevMap {
    pub entries: RevMapMappedSet,
    pub tags: RevMapMappedSet
}

impl RevMap {
    #[inline]
    pub fn new () -> RevMap {
        RevMap {
            entries: HashMap::new(),
            tags:    HashMap::new()
        }
    }

    /*
        Public API
    */

    #[inline]
    pub fn add (&mut self, entry: &str, tags: &[String]) {
        if !self.entries.contains_key(entry) {
            self.create_entry(entry);
        }

        self.ingest_list(&entry, &tags);
    }

    #[inline]
    pub fn remove (&mut self, entry: &str) {
        if let Some(entry_tags) = self.entries.get_mut(entry) {
            for tag in entry_tags.iter() {
                if let Some(tag_entries) = self.tags.get_mut(tag) {
                    tag_entries.remove(entry);
                }
            }
        }

        self.entries.remove(entry);
    }

    #[inline]
    pub fn get_entry_tags (&self, tag: &str) -> Option<&HashSet<String>> {
        self.entries.get(tag)
    }

    #[inline]
    pub fn get_tag_entries (&self, tag: &str) -> Option<&HashSet<String>> {
        self.tags.get(tag)
    }

    /*
        Private API
    */

    #[inline]
    fn create_entry (&mut self, entry: &str) {
        self.entries.insert(entry.to_string(), HashSet::new());
    }

    #[inline]
    fn create_tag_with_entry (tags: &mut RevMapMappedSet, tag: &str, entry: &str) {
        let mut tag_set = HashSet::new();

        tag_set.insert(entry.to_string());

        tags.insert(tag.to_string(), tag_set);
    }

    #[inline]
    fn ingest_list (&mut self, entry: &str, tags: &[String]) {
        if let Some(revmap_entry) = self.entries.get_mut(entry) {
            for tag in tags {

                if !revmap_entry.contains(tag) {
                    revmap_entry.insert(tag.to_string());
                }

                if let Some(mut_tag) = self.tags.get_mut(tag) {
                    if !mut_tag.contains(entry) {
                        mut_tag.insert(entry.to_string());
                    }

                    continue;
                }

                Self::create_tag_with_entry(&mut self.tags, tag, entry)
            }
        }
    }
}
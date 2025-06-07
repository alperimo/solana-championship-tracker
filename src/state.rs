use borsh::{BorshDeserialize, BorshSerialize};

/// Define struct representing Fenerbahçe's championship tracker
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FenerbahceTracker {
    pub total_trophies: u64,    // Total league championships
    pub current_season: u16,    // Current season year (e.g., 2010 for 2010-2011 season)
    pub seasons_played: u8,     // Number of seasons completed since 2010
}

impl FenerbahceTracker {
    pub const STARTING_SEASON: u16 = 2010;
    pub const ENDING_SEASON: u16 = 2024;
    pub const INITIAL_TROPHIES: u64 = 17;
    
    pub fn new() -> Self {
        Self {
            total_trophies: Self::INITIAL_TROPHIES,
            current_season: Self::STARTING_SEASON,
            seasons_played: 0,
        }
    }
    
    pub fn get_season_string(&self) -> String {
        format!("{}-{}", self.current_season, self.current_season + 1)
    }
    
    pub fn is_season_complete(&self) -> bool {
        self.current_season > Self::ENDING_SEASON
    }
}

/// Fenerbahçe's league positions from 2010-2011 to 2024-2025
pub struct SeasonData {
    pub season: u16,
    pub position: u8,
    pub champion: bool,
    pub points: u16,
    pub description: &'static str,
}

impl SeasonData {
    pub const SEASONS: [SeasonData; 15] = [
        SeasonData { season: 2010, position: 1, champion: true, points: 82, description: "🏆 CHAMPIONS! Title won under Aykut Kocaman, finished same point with Trabzonspor (82 pts)" },
        SeasonData { season: 2011, position: 2, champion: false, points: 68, description: "2nd place finish, 9 points behind champion Galatasaray (77 pts)" },
        SeasonData { season: 2012, position: 2, champion: false, points: 61, description: "2nd place finish, 10 points behind champion Galatasaray (71 pts)" },
        SeasonData { season: 2013, position: 1, champion: true, points: 74, description: "🏆 CHAMPIONS! Title won under Ersun Yanal, finished 9 points ahead of Galatasaray (65 pts)" },
        SeasonData { season: 2014, position: 2, champion: false, points: 74, description: "2nd place finish, 3 points behind champion Galatasaray (77 pts)" },
        SeasonData { season: 2015, position: 2, champion: false, points: 74, description: "2nd place finish, 5 points behind champion Beşiktaş (79 pts)" },
        SeasonData { season: 2016, position: 3, champion: false, points: 64, description: "3rd place finish, 13 points behind champion Beşiktaş (77 pts)" },
        SeasonData { season: 2017, position: 2, champion: false, points: 72, description: "2nd place finish, 3 points behind champion Galatasaray (75 pts)" },
        SeasonData { season: 2018, position: 6, champion: false, points: 46, description: "6th place finish, 23 points behind champion Galatasaray (69 pts)" },
        SeasonData { season: 2019, position: 7, champion: false, points: 53, description: "7th place finish, 13 points behind champion Başakşehir (66 pts)" },
        SeasonData { season: 2020, position: 3, champion: false, points: 82, description: "3rd place finish, tied on points with Galatasaray, 2 points behind champion Beşiktaş (84 pts)" },
        SeasonData { season: 2021, position: 2, champion: false, points: 73, description: "2nd place finish, 8 points behind champion Trabzonspor (81 pts)" },
        SeasonData { season: 2022, position: 2, champion: false, points: 80, description: "2nd place finish, 5 points behind champion Galatasaray (85 pts)" },
        SeasonData { season: 2023, position: 2, champion: false, points: 99, description: "2nd place finish despite a record 99 points, 3 points behind champion Galatasaray (102 pts)" },
        SeasonData { season: 2024, position: 2, champion: false, points: 84, description: "2nd place finish, 11 points behind champion Galatasaray (95 pts)" },
    ];
    
    pub fn get_season_data(season_year: u16) -> Option<&'static SeasonData> {
        Self::SEASONS.iter().find(|s| s.season == season_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenerbahce_tracker_serialization() {
        let original = FenerbahceTracker {
            total_trophies: 19,
            current_season: 2013,
            seasons_played: 3,
        };
        
        // Serialize using borsh directly
        let serialized = borsh::to_vec(&original).unwrap();
        
        // Deserialize
        let deserialized = FenerbahceTracker::try_from_slice(&serialized).unwrap();
        
        assert_eq!(original.total_trophies, deserialized.total_trophies);
        assert_eq!(original.current_season, deserialized.current_season);
        assert_eq!(original.seasons_played, deserialized.seasons_played);
    }

    #[test]
    fn test_fenerbahce_tracker_size() {
        let tracker = FenerbahceTracker::new();
        let serialized = borsh::to_vec(&tracker).unwrap();
        
        // Should be 8 bytes (u64) + 2 bytes (u16) + 1 byte (u8) = 11 bytes
        assert_eq!(serialized.len(), 11);
    }

    #[test]
    fn test_fenerbahce_tracker_new() {
        let tracker = FenerbahceTracker::new();
        
        assert_eq!(tracker.total_trophies, FenerbahceTracker::INITIAL_TROPHIES);
        assert_eq!(tracker.current_season, FenerbahceTracker::STARTING_SEASON);
        assert_eq!(tracker.seasons_played, 0);
    }

    #[test]
    fn test_fenerbahce_tracker_season_string() {
        let tracker = FenerbahceTracker {
            total_trophies: 17,
            current_season: 2010,
            seasons_played: 0,
        };
        
        assert_eq!(tracker.get_season_string(), "2010-2011");
        
        let tracker2 = FenerbahceTracker {
            total_trophies: 19,
            current_season: 2013,
            seasons_played: 3,
        };
        
        assert_eq!(tracker2.get_season_string(), "2013-2014");
    }

    #[test]
    fn test_fenerbahce_tracker_season_completion() {
        let mut tracker = FenerbahceTracker::new();
        assert!(!tracker.is_season_complete());
        
        tracker.current_season = 2024;
        assert!(!tracker.is_season_complete());
        
        tracker.current_season = 2025;
        assert!(tracker.is_season_complete());
    }

    #[test]
    fn test_season_data_lookup() {
        // Test championship seasons
        let season_2010 = SeasonData::get_season_data(2010).unwrap();
        assert_eq!(season_2010.position, 1);
        assert!(season_2010.champion);
        assert_eq!(season_2010.points, 82);
        
        let season_2013 = SeasonData::get_season_data(2013).unwrap();
        assert_eq!(season_2013.position, 1);
        assert!(season_2013.champion);
        assert_eq!(season_2013.points, 75);
        
        // Test non-championship season
        let season_2011 = SeasonData::get_season_data(2011).unwrap();
        assert_eq!(season_2011.position, 2);
        assert!(!season_2011.champion);
        assert_eq!(season_2011.points, 71);
        
        // Test invalid season
        assert!(SeasonData::get_season_data(2009).is_none());
        assert!(SeasonData::get_season_data(2030).is_none());
    }

    #[test]
    fn test_seasons_data_count() {
        assert_eq!(SeasonData::SEASONS.len(), 15);
        
        // Verify all seasons from 2010 to 2024 are present
        for year in 2010..=2024 {
            assert!(SeasonData::get_season_data(year).is_some(), 
                   "Season {} should be present", year);
        }
    }
}

use crate::structs::CommunityFollowerView;
use diesel::{result::Error, *};
use lemmy_db_schema::{
  newtypes::{CommunityId, PersonId},
  schema::{community, community_follower, person},
  source::{
    community::{Community, CommunitySafe},
    person::{Person, PersonSafe},
  },
  traits::{ToSafe, ViewToVec},
};

type CommunityFollowerViewTuple = (CommunitySafe, PersonSafe, Option<bool>);

impl CommunityFollowerView {
  pub fn for_community(conn: &PgConnection, community_id: CommunityId) -> Result<Vec<Self>, Error> {
    let res = community_follower::table
      .inner_join(community::table)
      .inner_join(person::table)
      .select((
        Community::safe_columns_tuple(),
        Person::safe_columns_tuple(),
        community_follower::pending,
      ))
      .filter(community_follower::community_id.eq(community_id))
      .order_by(community::title)
      .load::<CommunityFollowerViewTuple>(conn)?;

    Ok(Self::from_tuple_to_vec(res))
  }

  pub fn for_person(conn: &PgConnection, person_id: PersonId) -> Result<Vec<Self>, Error> {
    let res = community_follower::table
      .inner_join(community::table)
      .inner_join(person::table)
      .select((
        Community::safe_columns_tuple(),
        Person::safe_columns_tuple(),
        community_follower::pending,
      ))
      .filter(community_follower::person_id.eq(person_id))
      .order_by(community::title)
      .load::<CommunityFollowerViewTuple>(conn)?;

    Ok(Self::from_tuple_to_vec(res))
  }

  pub fn read(
    conn: &PgConnection,
    community_id: CommunityId,
    person_id: PersonId,
  ) -> Result<Self, Error> {
    let (community, follower, pending) = community_follower::table
      .inner_join(community::table)
      .inner_join(person::table)
      .select((
        Community::safe_columns_tuple(),
        Person::safe_columns_tuple(),
        community_follower::pending,
      ))
      .filter(community_follower::person_id.eq(person_id))
      .filter(community_follower::community_id.eq(community_id))
      .first::<CommunityFollowerViewTuple>(conn)?;

    Ok(Self {
      community,
      follower,
      pending,
    })
  }
}

impl ViewToVec for CommunityFollowerView {
  type DbTuple = CommunityFollowerViewTuple;
  fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
    items
      .iter()
      .map(|a| Self {
        community: a.0.to_owned(),
        follower: a.1.to_owned(),
        pending: a.2.to_owned(),
      })
      .collect::<Vec<Self>>()
  }
}

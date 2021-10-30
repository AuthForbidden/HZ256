// Copyright 2021 AuthForbidden
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub fn encode(pd_bytes: &Vec<u8>) -> String {
  let _pd_dict = "不人知山无风一日云古有何来天主中时花上斋水春月相为年生君长心自如白归此见秋行去清江在夜空下得高里未明多金门青客是处三寒子落声千家事玉雨今道远朝南前万出路我入飞东草城深尽与流新烟开思树别已地回马将色酒还谁欲西可应独闲成闻光向重作更同阳雪身石望看诗游香满海十愁老林书情衣从歌头难红百尘方复似楼能名龙旧莫黄犹分后言平气到五叶当鸟仙间发过亦几离松北王初起非意台外华边竹然大公者故随安暮梦吟之四宫醉少孤野国晚汉文枝世柳神露阴终所坐幽好霜若绿关临轻岁逢九河碧微波留泉连问芳半乡池须共岂影馀近期因翠经小曲庭乐照两惊吹断溪双遥动";
  
  let mut pd_data = String::new();

  for pd_byte in pd_bytes {
    pd_data.push(_pd_dict.chars().nth(*pd_byte as usize).unwrap());
  }

  return pd_data;
}

pub fn decode(pd_data: String) -> Vec<u8> {
  let _pd_dict = "不人知山无风一日云古有何来天主中时花上斋水春月相为年生君长心自如白归此见秋行去清江在夜空下得高里未明多金门青客是处三寒子落声千家事玉雨今道远朝南前万出路我入飞东草城深尽与流新烟开思树别已地回马将色酒还谁欲西可应独闲成闻光向重作更同阳雪身石望看诗游香满海十愁老林书情衣从歌头难红百尘方复似楼能名龙旧莫黄犹分后言平气到五叶当鸟仙间发过亦几离松北王初起非意台外华边竹然大公者故随安暮梦吟之四宫醉少孤野国晚汉文枝世柳神露阴终所坐幽好霜若绿关临轻岁逢九河碧微波留泉连问芳半乡池须共岂影馀近期因翠经小曲庭乐照两惊吹断溪双遥动";
  let mut pd_bytes:Vec<u8> = vec![];

  for pd_char in pd_data.chars() {
    let pd_byte: u8 = _pd_dict.chars().position(|c| c == pd_char).unwrap() as u8;
    pd_bytes.push(pd_byte);
  }

  return pd_bytes;
}

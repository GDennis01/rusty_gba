/*

 Esegui:
 mov r0,32
 cmp r0,32
 if !eval_cond(NE)
   passed
 else
   not passed

  // prende due array
  //ogni array è del tipo ((enum_reg,u32),(enum_cpsr,u32),enum_mode)
  //prima tupla è before, seconda after
  generate_test( ((string,string),enum_mode,[](enum_reg,u8),[](enum_cspr,bool)),(..))
  generate_test( ( ("MOV","TESTADD"),mode,[(r0,0),(r1,0)],[(val,0),(c,true)] ) , (...)): String(JSON)


   {
      description:{
        instr:"MOV BLA BLA",
        verbose:"TESTING ADD"
      },
      before:{
        mode:bla,
        reg:{
        r0:0
        r1:0
        r2:0
        r3:0
        r4:0
        r5:0
        r6:0
        r7:0
        r8:0
        r9:0
        r10:0
        r11:0
        r12:0
        r13:0
        r14:0
        r15:0
        },
        cpsr:{
          val:0
          C:0
          V:0
          N:0
          Z:1
        }
      },
      after:{
        mode:bla,
        reg:{
        r0:32
        r1:0
        r2:0
        r3:0
        r4:0
        r5:0
        r6:0
        r7:0
        r8:0
        r9:0
        r10:0
        r11:0
        r12:0
        r13:0
        r14:0
        r15:0
        },
        cpsr:{
          val:0
          C:0
          V:0
          N:0
          Z:0
        }
      }
   }

*/

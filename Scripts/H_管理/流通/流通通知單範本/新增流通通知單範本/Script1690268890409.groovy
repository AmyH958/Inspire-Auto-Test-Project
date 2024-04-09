import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.setText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a_'))

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1'))

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/a__1_2_3'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3'), '新增')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_'), '欄位名稱:')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__TextField'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__TextField'), '')

WebUI.setText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__TextField'), '測試流通範本_20230725')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1'), '通知單類型:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    'Due notice first\nDue notice repeat\nOverdue notice first\nOverdue notice repeat\nOverdue notice last\nReservation notice\nReservation canceled by lilbrarian\nReservation expired\nReservation canceled by patron\nReservation canceled - item not available\nReservation relink to title (item not available)\nHold notice first\nHold notice repeat\nHold canceled by patron\nHold canceled by librarian\nHold expired\nReset password\nShorted loan\nHold canceled - borrow to other patron\nHold temporary blocked (In inventory)\nNew patron card\nRecall Item\nReservation canceled - patron rights expired\nHold exceed times\nOverdue notice have Reservation\nOverdue notice penalty no paid\nNew librarian\nSDI notice\nReservation by another patron to borrow\nClaim return book already found notice\nClaim return book not found notice\nCallItem notice\nCallItem canceled by patron\nCallItem canceled by lilbrarian\nCallItem expired\nCallItem canceled - item not available\nRecommend order notice\nRecommend reject notice\nRecommend checkin notice\nRecommend Requested Notice\nPayments For Books Notification Report\nNewBook notice\nCurrent checkout notice\nBorrowing notice\nRenew result notice\nCheckin notice\nCards due notice\nModify duedate notice\nEqu reservation cancelled by patron\nEquipemnt check fail\nEquipment_check_ok\nEquipment_overdue\nEqu reservation non-report of suspension\nEqu reservation non-report of fine\nEqu reservation cancelled by utilizatori\nEqu reservation successful\nBespeak success\nCancel bespeak\nCourse cancel\nTeacher evaluation\nStart the course Remind\nRecommend transfer librarian notice\nReset Password Successfully')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    '3', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    'Due notice first\nDue notice repeat\nOverdue notice first\nOverdue notice repeat\nOverdue notice last\nReservation notice\nReservation canceled by lilbrarian\nReservation expired\nReservation canceled by patron\nReservation canceled - item not available\nReservation relink to title (item not available)\nHold notice first\nHold notice repeat\nHold canceled by patron\nHold canceled by librarian\nHold expired\nReset password\nShorted loan\nHold canceled - borrow to other patron\nHold temporary blocked (In inventory)\nNew patron card\nRecall Item\nReservation canceled - patron rights expired\nHold exceed times\nOverdue notice have Reservation\nOverdue notice penalty no paid\nNew librarian\nSDI notice\nReservation by another patron to borrow\nClaim return book already found notice\nClaim return book not found notice\nCallItem notice\nCallItem canceled by patron\nCallItem canceled by lilbrarian\nCallItem expired\nCallItem canceled - item not available\nRecommend order notice\nRecommend reject notice\nRecommend checkin notice\nRecommend Requested Notice\nPayments For Books Notification Report\nNewBook notice\nCurrent checkout notice\nBorrowing notice\nRenew result notice\nCheckin notice\nCards due notice\nModify duedate notice\nEqu reservation cancelled by patron\nEquipemnt check fail\nEquipment_check_ok\nEquipment_overdue\nEqu reservation non-report of suspension\nEqu reservation non-report of fine\nEqu reservation cancelled by utilizatori\nEqu reservation successful\nBespeak success\nCancel bespeak\nCourse cancel\nTeacher evaluation\nStart the course Remind\nRecommend transfer librarian notice\nReset Password Successfully')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/select_Due notice firstDue notice repeatOve_0d2428'), 
    '4', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1_2'), '寄送方式:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/span_Email'), 'Email')

WebUI.verifyElementClickable(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_Email_Checkbox'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_Email_Checkbox'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_Email_Checkbox'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_SMS_Checkbox_0'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_Print_Checkbox_1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_Internal_Checkbox_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/span_Print'), 'Print')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/span_Internal'), 'Internal')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1_2_3'), '預設值:')

WebUI.verifyElementClickable(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__RadioGroup'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__RadioGroup'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1_2_3_4'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Item number'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Item number'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Item number'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Start date time'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Start date time'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Start date time'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Start date time'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_End date'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Close date'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Patron username'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_Ptransaction id'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/span__1'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/span__1'))

WebUI.takeFullPageScreenshotAsCheckpoint('新增流通通知範本')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4_5'), '取消')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4_5'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4_5_6'))

WebUI.closeBrowser()


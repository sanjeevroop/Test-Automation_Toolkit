import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_Password'), '1234')

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/div_YOUR SETUP'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/button_Your Setup'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/a_Your Payroll'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/a_Payroll Basic Details'))

WebUI.selectOptionByValue(findTestObject('Screens/PayrollCostCodeFacets/select_testDev Payroll 1WebTes'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/button_Next'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/span_glyphicon glyphicon-send'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/button_Next_1'))

WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/button_Next_2'))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet0.Name'), findTestData('PayrollCostCodeFacets').getValue(1, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet0.Length'), findTestData('PayrollCostCodeFacets').getValue(2, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet1.Name'), findTestData('PayrollCostCodeFacets').getValue(3, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet1.Length'), findTestData('PayrollCostCodeFacets').getValue(4, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet2.Name'), findTestData('PayrollCostCodeFacets').getValue(5, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet2.Length'), findTestData('PayrollCostCodeFacets').getValue(6, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet3.Name'), findTestData('PayrollCostCodeFacets').getValue(7, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet3.Length'), findTestData('PayrollCostCodeFacets').getValue(8, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet4.Name'), findTestData('PayrollCostCodeFacets').getValue(9, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet4.Length'), findTestData('PayrollCostCodeFacets').getValue(10, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet5.Name'), findTestData('PayrollCostCodeFacets').getValue(11, 1))

WebUI.setText(findTestObject('Screens/PayrollCostCodeFacets/input_CostCodeFacet5.Length'), findTestData('PayrollCostCodeFacets').getValue(12, 1))

//WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/div_Data has been successfully'))
WebUI.click(findTestObject('Screens/PayrollCostCodeFacets/button_Save'))

//WebUI.closeBrowser()

